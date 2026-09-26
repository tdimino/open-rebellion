
undefined4 __thiscall FUN_00536300(void *this,undefined4 param_1,uint *param_2,void *param_3)

{
  uint *puVar1;
  int *piVar2;
  int iVar3;
  undefined4 uVar4;
  bool bVar5;
  void *local_10;
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00645b28;
  pvStack_c = ExceptionList;
  bVar5 = true;
  ExceptionList = &pvStack_c;
  local_10 = this;
  FUN_004ece30(&local_10);
  local_4 = 0;
  puVar1 = FUN_004ece40(param_2);
  if (puVar1 != (uint *)0x0) {
    piVar2 = (int *)FUN_00504dc0(param_2);
    if (piVar2 != (int *)0x0) {
      iVar3 = (**(code **)(*piVar2 + 0xc))(&local_10);
      bVar5 = iVar3 != 0;
    }
  }
  iVar3 = FUN_00534440(this,(int *)&local_10,param_3);
  if ((iVar3 == 0) || (!bVar5)) {
    uVar4 = 0;
  }
  else {
    uVar4 = 1;
  }
  local_4 = 0xffffffff;
  FUN_00619730();
  ExceptionList = pvStack_c;
  return uVar4;
}

