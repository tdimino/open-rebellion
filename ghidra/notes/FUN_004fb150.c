
int __thiscall FUN_004fb150(void *this,undefined4 param_1,undefined4 param_2,void *param_3)

{
  void *this_00;
  int iVar1;
  int iVar2;
  void *pvVar3;
  undefined4 local_2c [7];
  int local_10;
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00640068;
  pvStack_c = ExceptionList;
  iVar2 = 1;
  ExceptionList = &pvStack_c;
  FUN_004fcd00(local_2c,this,4);
  local_4 = 0;
  FUN_00513120((int)local_2c);
  while (local_10 != 0) {
    pvVar3 = param_3;
    this_00 = (void *)FUN_0052bed0((int)local_2c);
    iVar1 = FUN_004f80e0(this_00,pvVar3);
    if ((iVar1 == 0) || (iVar2 == 0)) {
      iVar2 = 0;
    }
    else {
      iVar2 = 1;
    }
    FUN_005130d0((int)local_2c);
  }
  local_4 = 0xffffffff;
  FUN_004fcd80(local_2c);
  ExceptionList = pvStack_c;
  return iVar2;
}

