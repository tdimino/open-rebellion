
undefined4 FUN_006015a0(int param_1,int param_2)

{
  if (*(int *)(param_2 + 0x18) <= *(int *)(param_1 + 0x18)) {
    if (*(int *)(param_2 + 0x18) < *(int *)(param_1 + 0x18)) {
      return 0;
    }
    if (*(int *)(param_2 + 0x20) <= *(int *)(param_1 + 0x20)) {
      if (*(int *)(param_2 + 0x20) < *(int *)(param_1 + 0x20)) {
        return 0;
      }
      if (*(int *)(param_2 + 0x1c) <= *(int *)(param_1 + 0x1c)) {
        if (*(int *)(param_2 + 0x1c) < *(int *)(param_1 + 0x1c)) {
          return 0;
        }
        if (*(int *)(param_2 + 0x24) <= *(int *)(param_1 + 0x24)) {
          return 0;
        }
      }
    }
  }
  return 1;
}

