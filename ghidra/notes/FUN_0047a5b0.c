
undefined4 __fastcall FUN_0047a5b0(int *param_1)

{
  uint *puVar1;
  int *this;
  bool bVar2;
  void *pvVar3;
  void *pvVar4;
  undefined4 *puVar5;
  int iVar6;
  uint local_24;
  undefined4 local_20;
  undefined1 *local_1c;
  undefined4 local_18;
  undefined4 local_14;
  undefined4 local_10;
  void *local_c;
  undefined1 *puStack_8;
  int local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_006334c8;
  local_c = ExceptionList;
  local_20 = 0;
  ExceptionList = &local_c;
  FUN_004ece30(&local_24);
  local_4 = 0;
  local_1c = (undefined1 *)0x30;
  param_1[8] = param_1[8] & 0xfffffffe;
  puVar1 = (uint *)(param_1 + 0xb);
  local_18 = 0x40;
  if (((uint)param_1[0xb] >> 0x18 < 0x30) || (0x3f < (uint)param_1[0xb] >> 0x18)) {
    bVar2 = false;
  }
  else {
    bVar2 = true;
  }
  FUN_00619730();
  if (bVar2) {
    pvVar3 = FUN_004f5940((void *)param_1[0x10],puVar1);
    if (pvVar3 != (void *)0x0) {
      if (*(int *)((int)pvVar3 + 0x28) == param_1[6]) {
        if ((*(byte *)((int)pvVar3 + 0x30) & 0x50) == 0) {
          param_1[8] = param_1[8] | 1;
        }
        else {
          *(uint *)((int)pvVar3 + 0x30) = *(uint *)((int)pvVar3 + 0x30) | 0x800000;
          *(undefined4 *)((int)pvVar3 + 0x2c) = 0;
          *(undefined4 *)((int)pvVar3 + 0x24) = 0;
          *(undefined4 *)((int)pvVar3 + 0x28) = 0;
          FUN_004ece80(puVar1);
          (**(code **)(*param_1 + 0x2c))();
        }
      }
      else {
        FUN_004ece80(puVar1);
      }
    }
  }
  if ((*(byte *)(param_1 + 8) & 1) == 0) {
    iVar6 = param_1[0x11];
    local_1c = &stack0xffffffc4;
    FUN_004f26d0(&stack0xffffffc4,param_1 + 10);
    pvVar3 = FUN_00403d30((void *)(iVar6 + 0x2c));
    if ((pvVar3 == (void *)0x0) || ((*(uint *)((int)pvVar3 + 0x30) & 0x20000000) != 0)) {
      FUN_004ece80(param_1 + 10);
    }
    else {
      pvVar4 = FUN_00403460((void *)param_1[0x10],0x800002,0x4000000,0x4f4000f1,0,6,0x32,10000,
                            (void *)0x1);
      this = param_1 + 0xc;
      FUN_00435790(this,(int)pvVar4);
      puVar5 = FUN_00403750((void *)param_1[0x10],param_1 + 10,0x800002,0x4000000,0x4f4000f1,0,2);
      FUN_00435790(this,(int)puVar5);
      puVar5 = FUN_004357b0(this,&local_1c);
      local_4._0_1_ = 1;
      FUN_004f26d0(&local_24,puVar5);
      while( true ) {
        local_4 = (uint)local_4._1_3_ << 8;
        FUN_00619730();
        local_14 = 0x30;
        local_10 = 0x40;
        if (((local_24 >> 0x18 < 0x30) || (0x3f < local_24 >> 0x18)) ||
           (bVar2 = true, (~*(byte *)(param_1 + 8) & 1) == 0)) {
          bVar2 = false;
        }
        FUN_00619730();
        if (!bVar2) break;
        pvVar4 = FUN_004f5940((void *)param_1[0x10],&local_24);
        if (pvVar4 != (void *)0x0) {
          iVar6 = FUN_004bc180(param_1,&local_24);
          if (iVar6 != 0) {
            FUN_004f26d0(param_1 + 0xb,&local_24);
            *(uint *)((int)pvVar4 + 0x30) = *(uint *)((int)pvVar4 + 0x30) & 0xff7fffff;
            *(undefined4 *)((int)pvVar4 + 0x2c) = 0;
            iVar6 = param_1[6];
            *(undefined4 *)((int)pvVar4 + 0x24) = 0;
            *(int *)((int)pvVar4 + 0x28) = iVar6;
            FUN_004021b0((int)pvVar4);
            param_1[8] = param_1[8] | 1;
            *(uint *)((int)pvVar3 + 0x30) = *(uint *)((int)pvVar3 + 0x30) | 0x20000000;
          }
        }
        puVar5 = FUN_004357b0(param_1 + 0xc,&local_1c);
        local_4._0_1_ = 2;
        FUN_004f26d0(&local_24,puVar5);
      }
      FUN_005f58b0(param_1 + 0xc);
    }
    if ((*(byte *)(param_1 + 8) & 1) == 0) goto LAB_0047a820;
  }
  local_20 = 1;
LAB_0047a820:
  local_4 = 0xffffffff;
  FUN_00619730();
  ExceptionList = local_c;
  return local_20;
}

