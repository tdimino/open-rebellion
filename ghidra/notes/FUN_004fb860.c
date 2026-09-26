
bool __thiscall FUN_004fb860(void *this,undefined4 param_1,int param_2)

{
  int iVar1;
  int *piVar2;
  bool bVar3;
  undefined4 local_2c [7];
  int local_10;
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00640108;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  iVar1 = FUN_004f9a60(this,param_2);
  bVar3 = iVar1 != 0;
  FUN_004fcd00(local_2c,this,4);
  local_4 = 0;
  FUN_00513120((int)local_2c);
  while (local_10 != 0) {
    piVar2 = (int *)FUN_0052bed0((int)local_2c);
    iVar1 = (**(code **)(*piVar2 + 0x8c))(param_2);
    if ((iVar1 == 0) || (!bVar3)) {
      bVar3 = false;
    }
    else {
      bVar3 = true;
    }
    FUN_005130d0((int)local_2c);
  }
  local_4 = 0xffffffff;
  FUN_004fcd80(local_2c);
  ExceptionList = pvStack_c;
  return bVar3;
}

