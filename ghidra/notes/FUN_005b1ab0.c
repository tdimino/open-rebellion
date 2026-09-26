
void __thiscall FUN_005b1ab0(int param_1,undefined4 param_2)

{
  uint uVar1;
  
  if (*(int *)(param_1 + 0x158) != 0) {
    *(int *)(param_1 + 0x158) = *(int *)(param_1 + 0x158) + -1;
    switch(param_2) {
    case 0:
      uVar1 = *(int *)(param_1 + 0x370) - 1;
      *(uint *)(param_1 + 0x370) = -(uint)(uVar1 != 0) & uVar1;
      break;
    case 1:
      uVar1 = *(int *)(param_1 + 0x37c) - 1;
      *(uint *)(param_1 + 0x37c) = -(uint)(uVar1 != 0) & uVar1;
      return;
    case 2:
      uVar1 = *(int *)(param_1 + 0x378) - 1;
      *(uint *)(param_1 + 0x378) = -(uint)(uVar1 != 0) & uVar1;
      return;
    case 3:
      uVar1 = *(int *)(param_1 + 0x36c) - 1;
      *(uint *)(param_1 + 0x36c) = -(uint)(uVar1 != 0) & uVar1;
      return;
    case 4:
      uVar1 = *(int *)(param_1 + 0x374) - 1;
      *(uint *)(param_1 + 0x374) = -(uint)(uVar1 != 0) & uVar1;
      return;
    }
  }
  return;
}

