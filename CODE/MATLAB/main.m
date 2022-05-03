%% Maintanence
clc
clear
close all

%% Project Setup
 s40 = [0 0 0 0;
        0 0 0 0;
        0 0 0 0;
        0 0 0 0];
 s41 = [0 1 0 0;
        3 0 0 1;
        4 0 0 2;
        0 0 4 0];
 s91 = [5 3 0 0 7 0 0 0 0;
        6 0 0 1 9 5 0 0 0;
        0 9 8 0 0 0 0 6 0;
        8 0 0 0 6 0 0 0 3;
        4 0 0 8 0 3 0 0 1;
        7 0 0 0 2 0 0 0 6;
        0 6 0 0 0 0 2 8 0;
        0 0 0 4 1 9 0 0 5;
        0 0 0 0 8 0 0 7 9];
 S41 = solve_sudoku(s41);
 S91 = solve_sudoku(s91);
 %% Funtions
 function solved = solve_sudoku(board)
    [m,~] = size(board);
    flat_board = reshape(board',1,m^2);
    print_sudoku(board)
    [Aeq,beq] = equality(m,flat_board);
    c = cost(m);
    x = linprog(c,[],[],Aeq,beq,zeros(m^3,1),ones(m^3,1));
    solved = convert_sudoku(x);
    print_sudoku(solved)
 end
 function [a,b] = equality(m,flat_board)
    [a_row,b_row] = row(m);
    [a_col,b_col] = col(m);
    [a_box,b_box] = box(m);
    [a_unique_row,b_unique_row] = unique_row(m);
    [a_unique_col,b_unique_col] = unique_col(m);
    [a_unique_box,b_unique_box] = unique_box(m);
    [a_single,b_single] = single_digit(m);
    [a_given,b_given] = given(m,flat_board);
    a = [a_row;a_col;a_box;a_given;a_single;a_unique_row;a_unique_col;a_unique_box];
    b = [b_row;b_col;b_box;b_given;b_single;b_unique_row;b_unique_col;b_unique_box];
 end
 function [a,b] = given(m,flat_board)
    a = zeros(m^3);
    b = zeros(m^3,1);
    for i = 1:m^2
        if flat_board(i) ~=0
            index = (i-1)*m + flat_board(i);
            a(index,index) = 1;
            b((i-1)*m+flat_board(i)) = 1;
        end
    end
 end
 function [a,b] = row(m)
    a = zeros(m,m^3);
    num_range = 1:m;
    nums = zeros(1,m^2);
    for i = 1:m
        nums(m*(i-1)+1:m*(i-1)+m) = num_range;
    end
    sum = m*(m + 1)/2;
    for i = 1:m
        a(i,:) = [zeros(1,(i-1)*m^2) nums zeros(1,m^2*(m-i))];
    end
    b = ones(m,1)*sum;
    %a = [a zeros(m,m^3)]; %Account for one norm
 end
 function [a, b] = col(m)
    a = zeros(m,m^3);
    num_range = 1:m;
    sum = m*(m + 1)/2;
    for i = 1:m
        i_offset = m*(i-1);
        for j = 1:m
            j_offset = m^2*(j-1);
            a(i,j_offset+1+i_offset:j_offset+m+i_offset) = num_range;
        end
    end
    b = ones(m,1)*sum;
    %a = [a zeros(m,m^3)]; %Account for one norm
 end
 function [a,b] = box(m)
    a = zeros(m,m^3);
    num_range = 1:m;
    sum = m*(m + 1)/2;
    n = sqrt(m);
    for i = 1:n
        i_offset = n*m^2*(i-1);
        for j = 1:n
            j_offset = n*m*(j-1);
            for k = 1:n
                k_offset = m^2*(k-1);
                for l = 1:n
                    l_offset = m*(l-1);
                    offset = i_offset + j_offset + k_offset + l_offset;
                    a(n*(i-1)+j,offset+1:offset+m) = num_range;
                end
            end
        end
    end
    b = ones(m,1)*sum;
    %a = [a zeros(m,m^3)]; %Account for one norm
 end
 function c = cost(m)
    c = ones(1,m^3);
 end
 function [a,b] = unique_row(m)
    a = zeros(m^2,m^3);
    for i = 1:m
        i_index = (i-1)*m^2;
        for j = 1:m
            j_index = m*(j-1);
            index = i_index + j_index;
            a(m*(i-1)+1:m*(i-1)+m,index+1:index+m) = eye(m);
        end
    end
    b = ones(m^2,1);
    %a = [a zeros(m,m^3)]; %Account for one norm
 end
 function [a,b] = unique_col(m)
    a = zeros(m^2,m^3);
    for i = 1:m
        nums = [zeros(m,(i-1)*m) eye(m) zeros(m,(m-i)*m)];
        i_index = (i-1)*m;
        for j = 1:m
            j_index = (j-1)*m^2;
            a(i_index+1:i_index+m,j_index+1:j_index+m^2) = nums;
        end
    end
    b = ones(m^2,1);
    %a = [a zeros(m,m^3)]; %Account for one norm
 end
 function [a,b] = unique_box(m)
    a = zeros(m^2,m^3);
    n = sqrt(m);
    for i = 1:n
        i_offset = n*m^2*(i-1); %Row * Cell
        for j = 1:n
            j_offset = n*m*(j-1); %Col * Cell
            index = (i-1)*m*n+(j-1)*m;
            for k = 1:n
                k_offset = m^2*(k-1); %Row
                for l = 1:n
                    l_offset = m*(l-1); %Cell
                    offset = i_offset + j_offset + k_offset + l_offset;
                    a(index+1:index+m,offset+1:offset+m) = eye(m);
                end
            end
        end
    end
    b = ones(m^2,1);
    %a = [a zeros(m,m^3)]; %Account for one norm
 end
 function print_sudoku(board)
    [m,~] = size(board);
    num_cells = sqrt(m);
    fprintf('\n') %Start on new row
    for i = 1:m %Loop through rows
        for j = 1:m
            fprintf('%i', board(i,j));
            if (mod(j,num_cells) == 0) && (j ~=0) && (j~=m)
                fprintf('|')
            end
        end
        fprintf('\n')
        if (mod(i,num_cells) == 0) && (i ~=0) && (i~= m)
            for k = 1:(m+num_cells-1)
                fprintf('-');
            end
            fprintf('\n')
        end
    end
    
 end
 function [a,b] = single_digit(m)
    nums = ones(1,m);
    a = zeros(m^2,m^3);
    b = ones(m^2,1);
    for i = 1:m^2
        a(i,(i-1)*m+1:(i-1)*m+m) = nums;
    end
 end
 function solved = convert_sudoku(x)
    m = uint64((length(x))^(1/3));
    sudoku = zeros(m^2,1);
    vals = 1:m;
    for i = 1:m^2
        for j = 1:m
            if x((i-1)*m+j) > 0
                sudoku(i) = vals(j);
            end
        end
    end
    solved = reshape(sudoku,m,m)';
 end