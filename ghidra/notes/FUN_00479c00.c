
undefined4 __fastcall FUN_00479c00(int *param_1)

{
  uint *puVar1;
  int *this;
  bool bVar2;
  void *pvVar3;
  undefined4 *puVar4;
  int iVar5;
  undefined4 uVar6;
  uint local_1c;
  undefined1 *local_18;
  undefined4 local_14;
  undefined4 local_10;
  void *local_c;
  undefined1 *puStack_8;
  int local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00633400;
  local_c = ExceptionList;
  local_10 = 0;
  ExceptionList = &local_c;
  FUN_004ece30(&local_1c);
  local_4 = 0;
  local_18 = (undefined1 *)0x30;
  param_1[8] = param_1[8] & 0xfffffffe;
  puVar1 = (uint *)(param_1 + 0xb);
  local_14 = 0x40;
  if (((uint)param_1[0xb] >> 0x18 < 0x30) || (0x3f < (uint)param_1[0xb] >> 0x18)) {
    bVar2 = false;
  }
  else {
    bVar2 = true;
  }
  FUN_00619730();
  if ((bVar2) && (pvVar3 = FUN_004f5940((void *)param_1[0x10],puVar1), pvVar3 != (void *)0x0)) {
    if (*(int *)((int)pvVar3 + 0x28) == param_1[6]) {
      if (((*(uint *)((int)pvVar3 + 0x30) & 0x50) == 0) &&
         ((*(uint *)((int)pvVar3 + 0x30) & 0x400) != 0)) {
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
  if ((*(byte *)(param_1 + 8) & 1) == 0) {
    iVar5 = param_1[0x11];
    local_18 = &stack0xffffffd0;
    FUN_004f26d0(&stack0xffffffd0,param_1 + 10);
    pvVar3 = FUN_00403d30((void *)(iVar5 + 0x2c));
    if ((pvVar3 == (void *)0x0) || ((*(byte *)((int)pvVar3 + 0x30) & 1) == 0)) {
      FUN_004ece80(param_1 + 10);
    }
    else {
      pvVar3 = FUN_00403460((void *)param_1[0x10],0x402,0,0x74000f1,0,6,0,10000,(void *)0x1);
      this = param_1 + 0xc;
      FUN_00435790(this,(int)pvVar3);
      puVar4 = (undefined4 *)FUN_004357b0(this,&local_18);
      local_4._0_1_ = 1;
      FUN_004f26d0(&local_1c,puVar4);
      local_4 = (uint)local_4._1_3_ << 8;
      FUN_00619730();
      FUN_005f58b0(this);
      local_18 = (undefined1 *)0x30;
      local_14 = 0x40;
      if ((local_1c >> 0x18 < 0x30) || (0x3f < local_1c >> 0x18)) {
        bVar2 = false;
      }
      else {
        bVar2 = true;
      }
      FUN_00619730();
      if (((bVar2) &&
          (pvVar3 = FUN_004f5940((void *)param_1[0x10],&local_1c), pvVar3 != (void *)0x0)) &&
         (iVar5 = FUN_004bc180(param_1,&local_1c), iVar5 != 0)) {
        FUN_004f26d0(param_1 + 0xb,&local_1c);
        *(uint *)((int)pvVar3 + 0x30) = *(uint *)((int)pvVar3 + 0x30) & 0xff7fffff;
        *(undefined4 *)((int)pvVar3 + 0x2c) = 0;
        iVar5 = param_1[6];
        *(undefined4 *)((int)pvVar3 + 0x24) = 0;
        *(int *)((int)pvVar3 + 0x28) = iVar5;
        FUN_004021b0((int)pvVar3);
        param_1[8] = param_1[8] | 1;
      }
    }
    uVar6 = local_10;
    if ((*(byte *)(param_1 + 8) & 1) == 0) goto LAB_00479e04;
  }
  uVar6 = 1;
LAB_00479e04:
  local_4 = 0xffffffff;
  FUN_00619730();
  ExceptionList = local_c;
  return uVar6;
}

