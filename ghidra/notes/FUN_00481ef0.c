
undefined4 __thiscall FUN_00481ef0(void *this,uint *param_1)

{
  void *pvVar1;
  undefined4 *puVar2;
  void *pvVar3;
  undefined4 uVar4;
  uint local_14;
  undefined1 local_10 [4];
  void *local_c;
  undefined1 *puStack_8;
  int local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00634220;
  local_c = ExceptionList;
  uVar4 = 1;
  ExceptionList = &local_c;
  FUN_004ece30(&local_14);
  local_4 = 0;
  pvVar1 = FUN_004035d0(*(void **)((int)this + 100),0,2,1,0x7404051,0,0,0x45,10000,(void *)0x1);
  pvVar3 = (void *)((int)this + 0x54);
  FUN_00435790(pvVar3,(int)pvVar1);
  puVar2 = FUN_004357b0(pvVar3,local_10);
  local_4._0_1_ = 1;
  FUN_004f26d0(&local_14,puVar2);
  local_4 = (uint)local_4._1_3_ << 8;
  FUN_00619730();
  FUN_005f58b0(pvVar3);
  pvVar3 = FUN_004f5940((void *)(*(int *)((int)this + 0x68) + 0x8c),&local_14);
  if (pvVar3 != (void *)0x0) {
    pvVar1 = FUN_004f5940((void *)(*(int *)((int)this + 0x68) + 0x8c),param_1);
    if ((pvVar1 != (void *)0x0) && (*(int *)((int)pvVar1 + 0x40) < *(int *)((int)pvVar3 + 0x40))) {
      uVar4 = 0;
    }
  }
  local_4 = 0xffffffff;
  FUN_00619730();
  ExceptionList = local_c;
  return uVar4;
}

