
int __thiscall FUN_004fd790(void *this,int param_1,int *param_2)

{
  void *this_00;
  int iVar1;
  int *piVar2;
  undefined4 local_2c [7];
  int local_10;
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_006402f8;
  pvStack_c = ExceptionList;
  iVar1 = 1;
  ExceptionList = &pvStack_c;
  *param_2 = 0;
  FUN_00502e30(local_2c,this,3,*(uint *)((int)this + 0x24) >> 6 & 3);
  local_4 = 0;
  FUN_00513120((int)local_2c);
  do {
    if ((local_10 == 0) || (*param_2 != 0)) break;
    iVar1 = param_1;
    piVar2 = param_2;
    this_00 = (void *)FUN_0052bed0((int)local_2c);
    iVar1 = FUN_005006f0(this_00,iVar1,piVar2);
    FUN_005130d0((int)local_2c);
  } while (iVar1 != 0);
  local_4 = 0xffffffff;
  FUN_00502eb0(local_2c);
  ExceptionList = pvStack_c;
  return iVar1;
}

