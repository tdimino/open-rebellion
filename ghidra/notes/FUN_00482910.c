
void __fastcall FUN_00482910(int param_1)

{
  int iVar1;
  bool bVar2;
  bool bVar3;
  uint uVar4;
  int *piVar5;
  void *pvVar6;
  uint *puVar7;
  void *this;
  undefined4 *puVar8;
  uint local_40;
  undefined1 local_3c [4];
  undefined1 *local_38;
  undefined1 *local_34;
  undefined4 local_30;
  undefined4 local_2c [7];
  int local_10;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00634340;
  local_c = ExceptionList;
  bVar3 = false;
  local_34 = (undefined1 *)0x90;
  uVar4 = *(uint *)(param_1 + 0x34) >> 0x18;
  local_30 = 0x98;
  if ((uVar4 < 0x90) || (0x97 < uVar4)) {
    bVar2 = false;
  }
  else {
    bVar2 = true;
  }
  ExceptionList = &local_c;
  FUN_00619730();
  if (bVar2) {
    piVar5 = FUN_004f3220(*(int *)(param_1 + 0x24),(uint *)(param_1 + 0x34));
    local_40 = *(uint *)(param_1 + 0x68);
    local_38 = &stack0xffffffa8;
    FUN_004f26d0(&stack0xffffffa8,(uint *)(param_1 + 0x34));
    pvVar6 = FUN_00403d30((void *)(local_40 + 0x2c));
    if (((piVar5 != (int *)0x0) && (pvVar6 != (void *)0x0)) &&
       ((*(uint *)((int)pvVar6 + 0x2c) & 0x1000000) != 0)) {
      FUN_004ffe70(local_2c,piVar5,1);
      local_4._0_1_ = 0;
      local_4._1_3_ = 0;
      FUN_00513120((int)local_2c);
      while ((local_10 != 0 && (!bVar3))) {
        puVar7 = &local_40;
        pvVar6 = (void *)FUN_0052bed0((int)local_2c);
        puVar7 = FUN_004025b0(pvVar6,puVar7);
        iVar1 = *(int *)(param_1 + 0x68);
        local_4._0_1_ = 1;
        local_34 = &stack0xffffffa8;
        FUN_004f26d0(&stack0xffffffa8,puVar7);
        pvVar6 = FUN_00419e40((void *)(iVar1 + 0x78));
        local_4._0_1_ = 0;
        FUN_00619730();
        if ((pvVar6 != (void *)0x0) &&
           (((*(uint *)((int)pvVar6 + 0x38) & 1) == 0 &&
            ((*(uint *)((int)pvVar6 + 0x38) & 0x10) != 0)))) {
          pvVar6 = (void *)thunk_FUN_005f5060((int)pvVar6 + 0x48);
          while ((pvVar6 != (void *)0x0 && (!bVar3))) {
            puVar7 = (uint *)FUN_00403040(pvVar6,local_3c);
            local_4._0_1_ = 2;
            this = FUN_004f5940((void *)(*(int *)(param_1 + 0x68) + 0x58),puVar7);
            local_4._0_1_ = 0;
            FUN_00619730();
            if ((this != (void *)0x0) && ((*(byte *)((int)this + 0x24) & 0x80) != 0)) {
              puVar8 = (undefined4 *)FUN_00403040(this,&local_38);
              local_4._0_1_ = 3;
              FUN_004f26d0((void *)(param_1 + 0x30),puVar8);
              local_4._0_1_ = 0;
              FUN_00619730();
              bVar3 = true;
            }
            pvVar6 = *(void **)((int)pvVar6 + 0x10);
          }
        }
        FUN_005130d0((int)local_2c);
      }
      local_4 = 0xffffffff;
      FUN_004fff70(local_2c);
    }
  }
  ExceptionList = local_c;
  return;
}

