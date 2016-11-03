
k = 5;
n = 2;
xlen = 14;
ylen = 36;

x = zeros(xlen,1);
x(1) = 1;

y = convenc(x, poly2trellis(k, [23, 33]));

save("-hdf5", "test_vectors/test_file.hdf", "k", "n", "xlen", "ylen", "x", "y");
%save("test_file.hdf", "k", "n", "xlen", "ylen", "x", "y");
