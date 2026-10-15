
int FUN_0055f0b0(uint *param_1,undefined4 param_2,undefined4 param_3)

{
  int *piVar1;
  uint uVar2;
  int iVar3;
  bool bVar4;
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 uStack_4;
  
  uStack_4 = 0xffffffff;
  puStack_8 = &LAB_00649f68;
  pvStack_c = ExceptionList;
  iVar3 = 1;
  ExceptionList = &pvStack_c;
  piVar1 = (int *)FUN_00504dc0(param_1);
  if (piVar1 != (int *)0x0) {
    uStack_4 = 0;
    uVar2 = (**(code **)(*piVar1 + 4))();
    if ((uVar2 < 0x30) || (0x3b < uVar2)) {
      iVar3 = 0;
    }
    uStack_4 = 0xffffffff;
    FUN_00619730();
    bVar4 = iVar3 != 0;
    iVar3 = 0;
    if (bVar4) {
      iVar3 = (**(code **)(*piVar1 + 0x2cc))(param_2,param_3);
    }
  }
  ExceptionList = pvStack_c;
  return iVar3;
}

