
uint * __thiscall FUN_004abe70(void *this,uint *param_1,int *param_2)

{
  bool bVar1;
  undefined3 extraout_var;
  undefined3 extraout_var_00;
  undefined4 *puVar2;
  uint uVar3;
  void *this_00;
  undefined3 extraout_var_01;
  uint *puVar4;
  uint *local_14;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0063849b;
  local_c = ExceptionList;
  local_14 = (uint *)0x0;
  if ((param_1 != (uint *)0x0) && (param_2 != (int *)0x0)) {
    ExceptionList = &local_c;
    bVar1 = FUN_005fc040((int *)param_1);
    if (CONCAT31(extraout_var,bVar1) != 0) {
      bVar1 = FUN_005fc040(param_2);
      if (CONCAT31(extraout_var_00,bVar1) != 0) {
        puVar2 = (undefined4 *)FUN_005fc0e0((int *)param_1);
        uVar3 = FUN_005fc0f0((int *)param_1);
        this_00 = (void *)FUN_00618b70(0x28);
        puVar4 = (uint *)0x0;
        local_4 = 0;
        if (this_00 != (void *)0x0) {
          puVar4 = FUN_005fbda0(this_00,(int)puVar2,uVar3,param_1);
        }
        local_4 = 0xffffffff;
        if (puVar4 != (uint *)0x0) {
          bVar1 = FUN_005fc040((int *)puVar4);
          if (CONCAT31(extraout_var_01,bVar1) != 0) {
            FUN_005fcc30(param_2,puVar4,0,0,*(int *)((int)this + 0x44),*(uint *)((int)this + 0x48),
                         puVar2,uVar3);
            FUN_005fd0f0(param_1,puVar4,0,0);
            local_14 = puVar4;
          }
        }
      }
    }
  }
  ExceptionList = local_c;
  return local_14;
}

