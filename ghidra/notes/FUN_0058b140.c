
int FUN_0058b140(int *param_1,int *param_2)

{
  if (*param_1 != 0) {
    if (*param_2 != 0) {
      if (param_2[1] < param_1[1]) {
        return 1;
      }
      return (param_2[1] <= param_1[1]) - 1;
    }
  }
  else if (*param_2 != 0) {
    return 1;
  }
  return -(uint)(*param_1 != 0);
}

