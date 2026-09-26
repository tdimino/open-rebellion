
void __thiscall FUN_00431450(void *this,void *param_1,void *param_2)

{
  void *pvVar1;
  uint *puVar2;
  undefined4 *puVar3;
  void *this_00;
  ushort uVar4;
  void **ppvVar5;
  undefined4 local_2c [7];
  int local_10;
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  pvVar1 = param_1;
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0062ce88;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  puVar2 = FUN_004025b0(param_1,(uint *)&param_1);
  uVar4 = 0;
  local_4 = 0;
  FUN_004f26d0((void *)((int)this + 0x18),puVar2);
  local_4 = 0xffffffff;
  FUN_00619730();
  puVar3 = (undefined4 *)FUN_00526560(pvVar1,&param_1);
  *(undefined4 *)((int)this + 0x40) = *puVar3;
  *(undefined4 *)((int)this + 0x30) = 0;
  *(undefined4 *)((int)this + 0x34) = 0;
  FUN_00512f90(local_2c,pvVar1,4);
  local_4 = 1;
  FUN_00513120((int)local_2c);
  pvVar1 = param_2;
  while (local_10 != 0) {
    ppvVar5 = &param_1;
    this_00 = (void *)FUN_0052bed0((int)local_2c);
    puVar2 = FUN_004025b0(this_00,(uint *)ppvVar5);
    local_4._0_1_ = 2;
    FUN_00432310(this,uVar4,puVar2,pvVar1);
    local_4 = CONCAT31(local_4._1_3_,1);
    FUN_00619730();
    FUN_005130d0((int)local_2c);
    uVar4 = uVar4 + 1;
  }
  *(undefined4 *)((int)this + 0x20) = 1;
  local_4 = 0xffffffff;
  FUN_00513010(local_2c);
  ExceptionList = pvStack_c;
  return;
}

