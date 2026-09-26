
undefined4 __thiscall FUN_004fae40(void *this,undefined4 param_1,undefined4 param_2,void *param_3)

{
  int iVar1;
  int *piVar2;
  uint uVar3;
  undefined4 uVar4;
  bool bVar5;
  undefined4 local_2c [7];
  int local_10;
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00640018;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  iVar1 = FUN_004f9a60(this,(int)param_3);
  bVar5 = iVar1 != 0;
  FUN_004fcd00(local_2c,this,3);
  local_4 = 0;
  FUN_00513120((int)local_2c);
  while (local_10 != 0) {
    piVar2 = (int *)FUN_0052bed0((int)local_2c);
    iVar1 = (**(code **)(*piVar2 + 0xb0))(param_3);
    if ((iVar1 == 0) || (!bVar5)) {
      bVar5 = false;
    }
    else {
      bVar5 = true;
    }
    FUN_005130d0((int)local_2c);
  }
  uVar3 = FUN_0053f950(0x302,this,param_1,param_2,param_3);
  if ((uVar3 == 0) || (!bVar5)) {
    bVar5 = false;
  }
  else {
    bVar5 = true;
  }
  uVar3 = FUN_0053f950(0x318,this,param_1,param_2,param_3);
  if ((uVar3 == 0) || (!bVar5)) {
    uVar4 = 0;
  }
  else {
    uVar4 = 1;
  }
  local_4 = 0xffffffff;
  FUN_004fcd80(local_2c);
  ExceptionList = pvStack_c;
  return uVar4;
}

