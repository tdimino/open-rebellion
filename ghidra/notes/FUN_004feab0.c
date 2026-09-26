
bool __thiscall FUN_004feab0(void *this,void *param_1)

{
  int iVar1;
  uint uVar2;
  undefined4 local_2c [7];
  int local_10;
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00640588;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  FUN_00502db0(local_2c,this,3);
  local_4 = 0;
  FUN_00513120((int)local_2c);
  for (uVar2 = 0; (local_10 != 0 && (uVar2 == 0)); uVar2 = uVar2 >> 9 & 1) {
    iVar1 = FUN_0052bed0((int)local_2c);
    uVar2 = *(uint *)(iVar1 + 0x50);
    FUN_005130d0((int)local_2c);
  }
  iVar1 = FUN_004f7800(this,uVar2,param_1);
  local_4 = 0xffffffff;
  FUN_00502eb0(local_2c);
  ExceptionList = pvStack_c;
  return iVar1 != 0;
}

