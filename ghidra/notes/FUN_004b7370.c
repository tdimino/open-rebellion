
void __thiscall FUN_004b7370(void *this,uint *param_1,undefined4 *param_2)

{
  bool bVar1;
  uint *this_00;
  uint uVar2;
  void *pvVar3;
  void *pvVar4;
  undefined4 *puVar5;
  undefined4 *puVar6;
  uint local_18;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  this_00 = param_1;
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00639600;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  FUN_004ece80(param_1);
  FUN_004ece80(param_2);
  uVar2 = *(uint *)(*(int *)((int)this + 0x10) + 0x34) >> 0x18;
  if ((uVar2 < 0x80) || (0x8f < uVar2)) {
    bVar1 = false;
  }
  else {
    bVar1 = true;
  }
  FUN_00619730();
  puVar6 = param_2;
  if (bVar1) {
    param_1 = *(uint **)((int)this + 0xc);
    puVar6 = (undefined4 *)0x4;
    FUN_004f26d0(&stack0xffffffd4,(undefined4 *)((int)this + 0x5c));
    pvVar3 = FUN_00403d30(param_1 + 0xb);
    if ((pvVar3 != (void *)0x0) && ((*(uint *)((int)pvVar3 + 0x2c) & 0xc00) != 0)) {
      FUN_004f26d0(this_00,(undefined4 *)((int)this + 0x5c));
    }
    if ((*this_00 >> 0x18 < 0x90) || (0x97 < *this_00 >> 0x18)) {
      bVar1 = false;
    }
    else {
      bVar1 = true;
    }
    param_1 = (uint *)CONCAT31(param_1._1_3_,!bVar1);
    FUN_00619730();
    if ((char)param_1 != '\0') {
      puVar6 = (undefined4 *)0x7000000;
      pvVar4 = FUN_0041a340(*(void **)((int)this + 0xc),
                            (undefined4 *)(*(int *)((int)this + 0x10) + 0x34),0,0x400,0x1000,0,0,0,
                            0x1d,&DAT_00000002);
      pvVar3 = (void *)((int)this + 0x18);
      FUN_00435790(pvVar3,(int)pvVar4);
      puVar5 = FUN_004357b0(pvVar3,&param_1);
      local_4 = 0;
      FUN_004f26d0(this_00,puVar5);
      local_4 = 0xffffffff;
      FUN_00619730();
      FUN_005f58b0(pvVar3);
    }
    local_18 = 0xc00;
    if ((*this_00 >> 0x18 < 0x90) || (0x97 < *this_00 >> 0x18)) {
      bVar1 = false;
    }
    else {
      bVar1 = true;
    }
    param_1 = (uint *)CONCAT31(param_1._1_3_,!bVar1);
    FUN_00619730();
    if ((char)param_1 != '\0') {
      puVar6 = (undefined4 *)0x7000000;
      local_18 = 0x800;
      pvVar4 = FUN_0041a340(*(void **)((int)this + 0xc),
                            (undefined4 *)(*(int *)((int)this + 0x10) + 0x34),0,0x800,0x1000,0,0,0,
                            0x1d,&DAT_00000002);
      pvVar3 = (void *)((int)this + 0x18);
      FUN_00435790(pvVar3,(int)pvVar4);
      puVar5 = FUN_004357b0(pvVar3,&param_1);
      local_4 = 1;
      FUN_004f26d0(this_00,puVar5);
      local_4 = 0xffffffff;
      FUN_00619730();
      FUN_005f58b0(pvVar3);
    }
    if ((*this_00 >> 0x18 < 0x90) || (0x97 < *this_00 >> 0x18)) {
      bVar1 = false;
    }
    else {
      bVar1 = true;
    }
    param_1 = (uint *)CONCAT31(param_1._1_3_,!bVar1);
    FUN_00619730();
    if ((char)param_1 != '\0') {
      puVar6 = (undefined4 *)0x4;
      local_18 = 0xc00;
      pvVar4 = FUN_0041a340(*(void **)((int)this + 0xc),
                            (undefined4 *)(*(int *)((int)this + 0x10) + 0x34),0,0x400,0,0,0,0,0x1d,
                            &DAT_00000002);
      pvVar3 = (void *)((int)this + 0x18);
      FUN_00435790(pvVar3,(int)pvVar4);
      puVar5 = FUN_004357b0(pvVar3,&param_1);
      local_4 = 2;
      FUN_004f26d0(this_00,puVar5);
      local_4 = 0xffffffff;
      FUN_00619730();
      FUN_005f58b0(pvVar3);
    }
    if ((*this_00 >> 0x18 < 0x90) || (0x97 < *this_00 >> 0x18)) {
      bVar1 = false;
    }
    else {
      bVar1 = true;
    }
    param_1 = (uint *)CONCAT31(param_1._1_3_,!bVar1);
    FUN_00619730();
    if ((char)param_1 != '\0') {
      local_18 = 0x800;
      pvVar4 = FUN_0041a340(*(void **)((int)this + 0xc),
                            (undefined4 *)(*(int *)((int)this + 0x10) + 0x34),0,0x800,0,0,0,0,0x1d,
                            &DAT_00000002);
      pvVar3 = (void *)((int)this + 0x18);
      FUN_00435790(pvVar3,(int)pvVar4);
      puVar6 = FUN_004357b0(pvVar3,&param_1);
      local_4 = 3;
      FUN_004f26d0(this_00,puVar6);
      local_4 = 0xffffffff;
      FUN_00619730();
      FUN_005f58b0(pvVar3);
      puVar6 = (undefined4 *)0x4;
    }
  }
  if ((*this_00 >> 0x18 < 0x90) || (0x97 < *this_00 >> 0x18)) {
    bVar1 = false;
  }
  else {
    bVar1 = true;
  }
  FUN_00619730();
  if (bVar1) {
    FUN_004af1c0(this,this_00,param_2,local_18,(uint)puVar6);
  }
  ExceptionList = local_c;
  return;
}

