
void * __thiscall FUN_00401d20(void *this,undefined1 *param_1,undefined4 param_2,undefined4 param_3)

{
  undefined1 *this_00;
  uint *puVar1;
  uint *puVar2;
  int *piVar3;
  uint uVar4;
  int iVar5;
  int iVar6;
  int extraout_ECX;
  int extraout_ECX_00;
  int extraout_ECX_01;
  undefined4 *puVar7;
  bool bVar8;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  this_00 = param_1;
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00629209;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  puVar1 = FUN_004025b0(param_1,(uint *)&param_1);
  local_4 = 0;
  FUN_004ecc70(this,puVar1);
  local_4._0_1_ = 2;
  FUN_00619730();
  puVar1 = (uint *)((int)this + 0x30);
  FUN_005f4950(puVar1,0);
  local_4._0_1_ = 3;
  FUN_005f4950((void *)((int)this + 0x34),0);
  local_4._0_1_ = 4;
  FUN_004ece30((undefined4 *)((int)this + 0x38));
  *(undefined4 *)((int)this + 0x1c) = param_2;
  *(undefined4 *)((int)this + 0x24) = 0;
  *(undefined4 *)((int)this + 0x28) = 0;
  *(undefined4 *)((int)this + 0x2c) = 0;
  *(undefined ***)this = &PTR_FUN_006584a0;
  *(undefined4 *)((int)this + 0x3c) = param_3;
  local_4._0_1_ = 5;
  puVar7 = (undefined4 *)((int)this + 0x40);
  for (iVar6 = 10; iVar6 != 0; iVar6 = iVar6 + -1) {
    *puVar7 = 0;
    puVar7 = puVar7 + 1;
  }
  *(uint *)((int)this + 0x20) = *(uint *)(this_00 + 0x24) >> 6 & 3;
  if (*(int *)((int)this + 0x1c) == 1) {
    param_1 = (undefined1 *)0x1;
  }
  else if (*(int *)((int)this + 0x1c) == 2) {
    param_1 = &DAT_00000002;
  }
  else {
    param_1 = (undefined1 *)0x0;
  }
  *puVar1 = *puVar1 | 0x800000;
  puVar2 = FUN_00403040(this,&param_2);
  local_4._0_1_ = 6;
  if ((*puVar2 >> 0x18 < 0x3c) || (0x3f < *puVar2 >> 0x18)) {
    bVar8 = false;
  }
  else {
    bVar8 = true;
  }
  FUN_00619730();
  local_4._0_1_ = 5;
  FUN_00619730();
  uVar4 = *puVar1;
  if (bVar8) {
    *puVar1 = uVar4 | 0x10000000;
    if (*(undefined1 **)((int)this + 0x20) == param_1) {
      *puVar1 = uVar4 | 0x10000002;
      piVar3 = FUN_00402d80(this_00,&param_1);
      local_4._0_1_ = 7;
      FUN_00402e80(this,piVar3);
      local_4._0_1_ = 5;
      FUN_00619730();
    }
    else {
      *puVar1 = *puVar1 | 4;
    }
    goto LAB_00401fd2;
  }
  *puVar1 = uVar4 | 0x20000000;
  if (*(undefined1 **)((int)this + 0x20) == param_1) {
    uVar4 = *puVar1;
    if ((this_00[0xac] & 1) == 0) {
LAB_00401eda:
      uVar4 = uVar4 | 2;
    }
    else {
      uVar4 = uVar4 | 4;
    }
  }
  else {
    uVar4 = *puVar1;
    if ((this_00[0xac] & 1) != 0) goto LAB_00401eda;
    uVar4 = uVar4 | 4;
  }
  *puVar1 = uVar4;
  if ((this_00[0xac] & 1) != 0) {
    *puVar1 = *puVar1 | 0x40;
  }
  uVar4 = FUN_005f4960((undefined4 *)(*(int *)((int)this + 0x3c) + 4));
  iVar6 = extraout_ECX;
  if ((uVar4 & 1) == 0) {
    param_1 = &stack0xffffffd4;
    FUN_00403040(this,&stack0xffffffd4);
    iVar5 = FUN_004024d0(iVar6);
    iVar6 = extraout_ECX_00;
    if (iVar5 != 0) {
      param_1 = (undefined1 *)0x34000280;
      *puVar1 = *puVar1 | 0x40000000;
      local_4._0_1_ = 8;
      piVar3 = FUN_00403040(this,&param_2);
      bVar8 = (undefined1 *)*piVar3 == param_1;
      FUN_00619730();
      local_4._0_1_ = 5;
      FUN_00619730();
      iVar6 = extraout_ECX_01;
      if (bVar8) {
        *puVar1 = *puVar1 | 0x400;
      }
    }
  }
  param_1 = &stack0xffffffd4;
  FUN_00403040(this,&stack0xffffffd4);
  iVar6 = FUN_004025f0(iVar6);
  if (iVar6 != 0) {
    *puVar1 = *puVar1 | 8;
  }
  if ((*(byte *)puVar1 & 8) == 0) {
    iVar6 = FUN_004ed1c0((int)this_00);
    if (iVar6 != 0) {
      *(uint *)((int)this + 0x34) = *(uint *)((int)this + 0x34) | 0x1000000;
    }
    iVar6 = FUN_004ed1e0((int)this_00);
    if (iVar6 != 0) {
      *(uint *)((int)this + 0x34) = *(uint *)((int)this + 0x34) | 0x2000000;
    }
    iVar6 = FUN_004ed200((int)this_00);
    if (iVar6 != 0) {
      *(uint *)((int)this + 0x34) = *(uint *)((int)this + 0x34) | 0x4000000;
    }
  }
LAB_00401fd2:
  FUN_00402720(this);
  ExceptionList = local_c;
  return this;
}

