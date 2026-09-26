
int * __fastcall FUN_004e0750(int param_1)

{
  int iVar1;
  void *pvVar2;
  bool bVar3;
  ushort uVar4;
  void *this;
  undefined4 *puVar5;
  void *pvVar6;
  int *piVar7;
  undefined1 *this_00;
  void *pvVar8;
  uint *puVar9;
  int *local_50;
  undefined1 local_48 [4];
  undefined1 local_44 [4];
  undefined1 local_40 [4];
  undefined4 local_3c [2];
  uint local_34;
  undefined1 *local_30;
  undefined4 local_2c [7];
  int local_10;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0063da4a;
  local_c = ExceptionList;
  pvVar8 = (void *)(param_1 + 0x7c);
  local_50 = (int *)0x0;
  bVar3 = false;
  ExceptionList = &local_c;
  this = (void *)thunk_FUN_005f5060((int)pvVar8);
  while (this != (void *)0x0) {
    if (bVar3) goto LAB_004e0908;
    puVar5 = FUN_00403040(this,local_48);
    local_30 = &stack0xffffff98;
    local_4 = 0;
    iVar1 = *(int *)(param_1 + 0x2c);
    FUN_004f26d0(&stack0xffffff98,puVar5);
    pvVar6 = FUN_00403d30((void *)(iVar1 + 0x2c));
    local_4 = 0xffffffff;
    FUN_00619730();
    bVar3 = false;
    if (pvVar6 != (void *)0x0) {
      uVar4 = FUN_005f50e0((int)pvVar8);
      if (*(int *)((int)pvVar6 + 0x5c) < 1) {
        pvVar2 = *(void **)((int)this + 0x10);
        puVar9 = FUN_00403040(this,&local_34);
        local_4 = 4;
        FUN_004f58e0(pvVar8,puVar9);
        local_4 = 0xffffffff;
        FUN_00619730();
        *(uint *)((int)pvVar6 + 0x28) = *(uint *)((int)pvVar6 + 0x28) & 0xffbfffff;
        this = pvVar2;
      }
      else if ((*(uint *)((int)pvVar6 + 0x28) & 0x1000) == 0) {
        this = *(void **)((int)this + 0x10);
      }
      else if (*(int *)((int)pvVar6 + 0x5c) < 2) {
        bVar3 = 1 < uVar4;
        if (bVar3) {
          puVar5 = FUN_00403040(this,local_40);
          local_4 = 2;
          FUN_004f26d0((void *)(param_1 + 0x58),puVar5);
          local_4 = 0xffffffff;
          FUN_00619730();
          puVar9 = FUN_00403040(this,local_3c);
          local_4 = 3;
          FUN_004f58e0(pvVar8,puVar9);
          local_4 = 0xffffffff;
          FUN_00619730();
          *(uint *)((int)pvVar6 + 0x28) = *(uint *)((int)pvVar6 + 0x28) & 0xffbfffff;
        }
        this = (void *)0x0;
      }
      else {
        puVar5 = FUN_00403040(this,local_44);
        local_4 = 1;
        FUN_004f26d0((void *)(param_1 + 0x58),puVar5);
        local_4 = 0xffffffff;
        FUN_00619730();
        bVar3 = true;
      }
    }
  }
  if (bVar3) {
LAB_004e0908:
    FUN_004f4340(local_3c);
    local_4 = 5;
    piVar7 = FUN_004f3220(*(int *)(param_1 + 0x30),(uint *)(param_1 + 0x58));
    if (piVar7 != (int *)0x0) {
      FUN_0052c740(local_2c,piVar7,1);
      local_4._0_1_ = 6;
      FUN_00513120((int)local_2c);
      if (local_10 != 0) {
        this_00 = (undefined1 *)FUN_00618b70(0x20);
        local_4._0_1_ = 7;
        local_30 = this_00;
        if (this_00 == (undefined1 *)0x0) {
          pvVar8 = (void *)0x0;
        }
        else {
          puVar9 = &local_34;
          pvVar8 = (void *)FUN_0052bed0((int)local_2c);
          puVar9 = FUN_004025b0(pvVar8,puVar9);
          local_4 = CONCAT31(local_4._1_3_,8);
          pvVar8 = FUN_004f5b10(this_00,puVar9,0);
        }
        local_4 = 7;
        if (this_00 != (undefined1 *)0x0) {
          FUN_00619730();
        }
        local_4._0_1_ = 6;
        if (pvVar8 != (void *)0x0) {
          FUN_004f57b0(local_3c,pvVar8);
          local_50 = (int *)FUN_004f5cd0(0x200);
          if (local_50 != (int *)0x0) {
            local_50[8] = *(int *)(param_1 + 0x30);
            (**(code **)(*local_50 + 0x24))();
          }
        }
      }
      local_4 = CONCAT31(local_4._1_3_,5);
      FUN_0052c840(local_2c);
    }
    local_4 = 0xffffffff;
    FUN_004f4380(local_3c);
  }
  ExceptionList = local_c;
  return local_50;
}

