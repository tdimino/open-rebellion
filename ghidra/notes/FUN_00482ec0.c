
void __fastcall FUN_00482ec0(void *param_1)

{
  bool bVar1;
  ushort uVar2;
  void *pvVar3;
  void *pvVar4;
  uint uVar5;
  undefined4 *puVar6;
  int iVar7;
  int iVar8;
  uint local_20;
  int local_1c;
  undefined1 *local_18;
  undefined4 local_14;
  undefined4 local_10;
  void *local_c;
  undefined1 *puStack_8;
  uint local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_006343f0;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  FUN_004ece30(&local_20);
  local_18 = &stack0xffffffcc;
  iVar8 = 0;
  *(uint *)((int)param_1 + 0x20) = *(uint *)((int)param_1 + 0x20) & 0xfffffffd;
  iVar7 = *(int *)((int)param_1 + 0x68);
  local_4 = 0;
  local_1c = 0;
  FUN_004f26d0(&stack0xffffffcc,(undefined4 *)((int)param_1 + 0x34));
  pvVar3 = FUN_00403d30((void *)(iVar7 + 0x2c));
  if (pvVar3 != (void *)0x0) {
    iVar8 = (*(int *)((int)pvVar3 + 0xbc) + 2) / 2;
    local_1c = iVar8;
  }
  uVar2 = FUN_005f50e0((int)param_1 + 0x44);
  if ((int)(uint)uVar2 < iVar8) {
    pvVar4 = FUN_00403460(*(void **)((int)param_1 + 100),0x10800002,*(uint *)((int)param_1 + 0x7c),
                          0x7400051,0,1,0x33,10000,(void *)0x1);
    pvVar3 = (void *)((int)param_1 + 0x54);
    FUN_00435790(pvVar3,(int)pvVar4);
    uVar5 = *(uint *)((int)param_1 + 0x38) >> 0x18;
    local_14 = 0x90;
    local_10 = 0x98;
    if ((uVar5 < 0x90) || (local_18 = (undefined1 *)0x1, 0x97 < uVar5)) {
      local_18 = (undefined1 *)0x0;
    }
    FUN_00619730();
    if (local_18 != (undefined1 *)0x0) {
      puVar6 = FUN_00403750(*(void **)((int)param_1 + 100),(undefined4 *)((int)param_1 + 0x38),
                            0x10800002,*(uint *)((int)param_1 + 0x7c),0x7400051,0,2);
      FUN_00435790(pvVar3,(int)puVar6);
    }
    puVar6 = FUN_004357b0(pvVar3,&local_18);
    local_4._0_1_ = 1;
    FUN_004f26d0(&local_20,puVar6);
    local_4 = (uint)local_4._1_3_ << 8;
    FUN_00619730();
    do {
      local_14 = 0x30;
      local_10 = 0x40;
      local_4 = CONCAT31(local_4._1_3_,2);
      if ((local_20 >> 0x18 < 0x30) || (0x3f < local_20 >> 0x18)) {
LAB_0048304d:
        bVar1 = false;
      }
      else {
        uVar2 = FUN_005f50e0((int)param_1 + 0x44);
        if (iVar8 <= (int)(uint)uVar2) goto LAB_0048304d;
        bVar1 = true;
      }
      local_4 = local_4 & 0xffffff00;
      FUN_00619730();
      if (!bVar1) goto LAB_004830af;
      iVar7 = FUN_0047ba90(param_1,&local_20);
      if (iVar7 != 0) {
        FUN_0047b730(param_1,&local_20);
      }
      puVar6 = FUN_004357b0(pvVar3,&local_18);
      local_4._0_1_ = 3;
      FUN_004f26d0(&local_20,puVar6);
      local_4 = (uint)local_4._1_3_ << 8;
      FUN_00619730();
      iVar8 = local_1c;
    } while( true );
  }
LAB_004830ba:
  uVar2 = FUN_005f50e0((int)param_1 + 0x44);
  if (iVar8 <= (int)(uint)uVar2) {
    *(uint *)((int)param_1 + 0x20) = *(uint *)((int)param_1 + 0x20) | 2;
    uVar2 = FUN_005f50e0((int)param_1 + 0x44);
    if (iVar8 < (int)(uint)uVar2) {
      FUN_0047b9e0(param_1);
    }
  }
  local_4 = 0xffffffff;
  FUN_00619730();
  ExceptionList = local_c;
  return;
LAB_004830af:
  FUN_005f58b0(pvVar3);
  iVar8 = local_1c;
  goto LAB_004830ba;
}

