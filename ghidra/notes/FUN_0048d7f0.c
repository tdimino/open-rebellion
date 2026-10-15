
void __thiscall FUN_0048d7f0(int param_1,int param_2)

{
  int iVar1;
  bool bVar2;
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 uStack_4;
  
  puStack_8 = &LAB_00635698;
  pvStack_c = ExceptionList;
  if (param_2 != 2) {
    if (param_2 == 0x211) {
      *(uint *)(param_1 + 100) = *(uint *)(param_1 + 100) | 1;
      return;
    }
    if (param_2 != 0x220) {
      return;
    }
    *(uint *)(param_1 + 100) = *(uint *)(param_1 + 100) | 0x10;
    return;
  }
  if ((*(uint *)(param_1 + 100) & 0x10000000) != 0) {
    *(undefined4 *)(param_1 + 0x40) = 0;
    return;
  }
  ExceptionList = &pvStack_c;
  *(undefined4 *)(param_1 + 0x40) = 1;
  if ((*(uint *)(param_1 + 100) & 0x10) != 0) {
    param_2 = 0x32000242;
    uStack_4 = 0;
    FUN_004f26d0(&param_2);
    uStack_4 = 0xffffffff;
    FUN_00619730();
    FUN_0048e3c0();
    goto LAB_0048d981;
  }
  iVar1 = *(int *)(param_1 + 0x68);
  uStack_4 = 1;
  if ((iVar1 == 0x32000242) || (FUN_00619730(), iVar1 == 0x31000241)) {
    iVar1 = *(int *)(param_1 + 0x6c);
    uStack_4 = CONCAT31(uStack_4._1_3_,2);
    if ((iVar1 == 0x35000281) || (FUN_00619730(), iVar1 == 0x34000280)) {
      bVar2 = true;
    }
    else {
      bVar2 = false;
    }
    uStack_4 = CONCAT31(uStack_4._1_3_,1);
    FUN_00619730();
    if (!bVar2) goto LAB_0048d94c;
    bVar2 = true;
  }
  else {
LAB_0048d94c:
    bVar2 = false;
  }
  uStack_4 = 0xffffffff;
  FUN_00619730();
  if (bVar2) {
    if ((*(byte *)(param_1 + 100) & 1) == 0) {
      FUN_0048df20();
    }
    else {
      FUN_0048dc50();
    }
  }
  else {
    FUN_0048e1d0();
  }
LAB_0048d981:
  FUN_0048db30();
  ExceptionList = pvStack_c;
  return;
}

