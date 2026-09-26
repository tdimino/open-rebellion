
uint __thiscall FUN_0050e200(void *this,int *param_1,void *param_2)

{
  void *pvVar1;
  bool bVar2;
  undefined3 extraout_var;
  int *piVar3;
  uint uVar4;
  undefined4 local_28 [5];
  undefined4 local_14;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  pvVar1 = param_2;
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00641fb0;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  bVar2 = FUN_004f9190(this,param_1,param_2);
  uVar4 = 0;
  if (CONCAT31(extraout_var,bVar2) != 0) {
    param_2 = (void *)0xa2000006;
    local_4 = 0;
    piVar3 = FUN_004f7d50(this,(uint *)&param_2,*(uint *)((int)this + 0x24) >> 6 & 3,pvVar1,1,1);
    local_4 = 0xffffffff;
    FUN_00619730();
    uVar4 = 0;
    if (piVar3 != (int *)0x0) {
      param_2 = (void *)0xa4000007;
      local_4 = 1;
      piVar3 = FUN_004f7d50(this,(uint *)&param_2,*(uint *)((int)this + 0x24) >> 6 & 3,pvVar1,1,1);
      local_4 = 0xffffffff;
      FUN_00619730();
      uVar4 = 0;
      if (piVar3 != (int *)0x0) {
        param_2 = (void *)0xa0000005;
        local_4 = 2;
        piVar3 = FUN_004f7d50(this,(uint *)&param_2,*(uint *)((int)this + 0x24) >> 6 & 3,pvVar1,1,1)
        ;
        local_4 = 0xffffffff;
        FUN_00619730();
        uVar4 = 0;
        if (piVar3 != (int *)0x0) {
          FUN_00540230((void *)(*(int *)((int)this + 0x54) + 0x18),DAT_006bb3a0);
          FUN_00540270((void *)(*(int *)((int)this + 0x54) + 0x18),DAT_006bb3a4);
          FUN_00540230((void *)(*(int *)((int)this + 0x54) + 0x20),DAT_006bb3c4);
          FUN_00540230((void *)(*(int *)((int)this + 0x54) + 0x28),DAT_006bb470);
          FUN_00540230((void *)(*(int *)((int)this + 0x54) + 0x30),DAT_006bb3d8);
          FUN_00540230((void *)(*(int *)((int)this + 0x54) + 0x38),DAT_006bb420);
          FUN_00540270((void *)(*(int *)((int)this + 0x54) + 0x38),DAT_006bb49c);
          FUN_00540230((void *)(*(int *)((int)this + 0x54) + 0x40),DAT_006bb3d0);
          FUN_00540270((void *)(*(int *)((int)this + 0x54) + 0x40),DAT_006bb484);
          FUN_004fd450(local_28,(int)pvVar1);
          local_4 = 3;
          local_14 = FUN_004fd2f0();
          uVar4 = FUN_0050c820(this,3,(int)param_1,local_28);
          local_4 = 0xffffffff;
          FUN_004fd4d0(local_28);
        }
      }
    }
  }
  ExceptionList = local_c;
  return uVar4;
}

