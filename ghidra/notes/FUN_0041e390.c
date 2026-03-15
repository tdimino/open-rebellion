
undefined4 __fastcall FUN_0041e390(int param_1)

{
  bool bVar1;
  int iVar2;
  undefined3 extraout_var;
  int iVar3;
  undefined3 extraout_var_00;
  uint uVar4;
  
  iVar2 = FUN_004010d0();
  uVar4 = 0;
  bVar1 = FUN_0041e1a0();
  if (CONCAT31(extraout_var,bVar1) != 0) {
    iVar3 = FUN_0051ce00();
    uVar4 = FUN_0051de80(iVar3);
  }
  if (iVar2 != 0) {
    if (uVar4 != 0) goto LAB_0041e3fe;
    if (*(int *)(param_1 + 0x14) == 0) {
      iVar2 = FUN_004010d0();
      if (iVar2 != 0) {
        FUN_004010f0();
      }
      bVar1 = FUN_0041e1a0();
      if (CONCAT31(extraout_var_00,bVar1) != 0) {
        iVar2 = FUN_0051ce00();
        FUN_0051dea0(iVar2);
      }
      return 1;
    }
  }
  if (uVar4 == 0) {
    return 0;
  }
LAB_0041e3fe:
  FUN_0041db60();
  return 0;
}

