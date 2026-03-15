
undefined4 __fastcall FUN_0051dea0(int param_1)

{
  uint uVar1;
  int iVar2;
  int iVar3;
  
  uVar1 = FUN_0051de80(param_1);
  if (uVar1 == 0) {
    *(int *)(param_1 + 0xc) = *(int *)(param_1 + 0xc) + 1;
    if (*(int *)(param_1 + 0x24) == 0) {
      *(int *)(param_1 + 0x20) = *(int *)(param_1 + 0x20) + 1;
    }
    else {
      *(undefined4 *)(param_1 + 0x24) = 0;
      *(undefined4 *)(param_1 + 0x20) = 0;
      *(int *)(param_1 + 0x10) = *(int *)(param_1 + 0x10) + 1;
    }
    iVar2 = FUN_00506f30(1);
    iVar3 = FUN_00506f30(2);
    if ((iVar2 != 0) && (iVar3 != 0)) {
      if (*(uint *)(iVar3 + 0xc4) < *(uint *)(iVar2 + 0xc4)) {
        *(uint *)(param_1 + 0x1c) = *(uint *)(iVar2 + 0xc4);
      }
      else {
        *(uint *)(param_1 + 0x1c) = *(uint *)(iVar3 + 0xc4);
      }
    }
    *(uint *)(param_1 + 0x24) = (uint)(*(uint *)(param_1 + 0x1c) <= *(uint *)(param_1 + 0x20));
  }
  return 1;
}

