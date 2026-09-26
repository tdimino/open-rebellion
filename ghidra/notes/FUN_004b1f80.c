
void __thiscall FUN_004b1f80(void *this,uint *param_1,undefined4 *param_2)

{
  void *this_00;
  bool bVar1;
  uint *this_01;
  uint uVar2;
  void *pvVar3;
  undefined4 *puVar4;
  uint uVar5;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  this_01 = param_1;
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00639048;
  local_c = ExceptionList;
  uVar5 = 0;
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
  if (bVar1) {
    uVar5 = 0x40000;
    pvVar3 = FUN_0041a340(*(void **)((int)this + 0xc),
                          (undefined4 *)(*(int *)((int)this + 0x10) + 0x34),0,0x40000,0x100,0,0,0,
                          0x1d,&DAT_00000002);
    this_00 = (void *)((int)this + 0x18);
    FUN_00435790(this_00,(int)pvVar3);
    puVar4 = FUN_004357b0(this_00,&param_1);
    local_4 = 0;
    FUN_004f26d0(this_01,puVar4);
    local_4 = 0xffffffff;
    FUN_00619730();
    FUN_005f58b0(this_00);
    if ((*this_01 >> 0x18 < 0x90) || (0x97 < *this_01 >> 0x18)) {
      bVar1 = false;
    }
    else {
      bVar1 = true;
    }
    param_1 = (uint *)CONCAT31(param_1._1_3_,!bVar1);
    FUN_00619730();
    if ((char)param_1 != '\0') {
      uVar5 = 0x80000;
      pvVar3 = FUN_0041a340(*(void **)((int)this + 0xc),
                            (undefined4 *)(*(int *)((int)this + 0x10) + 0x34),0,0x80000,0,0,0,0,0x1d
                            ,&DAT_00000002);
      FUN_00435790(this_00,(int)pvVar3);
      puVar4 = FUN_004357b0(this_00,&param_1);
      local_4 = 1;
      FUN_004f26d0(this_01,puVar4);
      local_4 = 0xffffffff;
      FUN_00619730();
      FUN_005f58b0(this_00);
    }
    if ((*this_01 >> 0x18 < 0x90) || (0x97 < *this_01 >> 0x18)) {
      bVar1 = false;
    }
    else {
      bVar1 = true;
    }
    param_1 = (uint *)CONCAT31(param_1._1_3_,!bVar1);
    FUN_00619730();
    if ((char)param_1 != '\0') {
      uVar5 = 0x100000;
      pvVar3 = FUN_0041a340(*(void **)((int)this + 0xc),
                            (undefined4 *)(*(int *)((int)this + 0x10) + 0x34),0,0x100000,0,0,0,0,
                            0x1d,&DAT_00000002);
      FUN_00435790(this_00,(int)pvVar3);
      puVar4 = FUN_004357b0(this_00,&param_1);
      local_4 = 2;
      FUN_004f26d0(this_01,puVar4);
      local_4 = 0xffffffff;
      FUN_00619730();
      FUN_005f58b0(this_00);
    }
    if ((*this_01 >> 0x18 < 0x90) || (0x97 < *this_01 >> 0x18)) {
      bVar1 = false;
    }
    else {
      bVar1 = true;
    }
    param_1 = (uint *)CONCAT31(param_1._1_3_,!bVar1);
    FUN_00619730();
    if ((char)param_1 != '\0') {
      uVar5 = 0x40000;
      pvVar3 = FUN_0041a340(*(void **)((int)this + 0xc),
                            (undefined4 *)(*(int *)((int)this + 0x10) + 0x34),0,0x40000,0,0,0,0,0x1d
                            ,&DAT_00000002);
      FUN_00435790(this_00,(int)pvVar3);
      puVar4 = FUN_004357b0(this_00,&param_1);
      local_4 = 3;
      FUN_004f26d0(this_01,puVar4);
      local_4 = 0xffffffff;
      FUN_00619730();
      FUN_005f58b0(this_00);
    }
    if ((*this_01 >> 0x18 < 0x90) || (0x97 < *this_01 >> 0x18)) {
      bVar1 = false;
    }
    else {
      bVar1 = true;
    }
    param_1 = (uint *)CONCAT31(param_1._1_3_,!bVar1);
    FUN_00619730();
    if ((char)param_1 != '\0') {
      uVar5 = 0x200000;
      pvVar3 = FUN_0041a340(*(void **)((int)this + 0xc),
                            (undefined4 *)(*(int *)((int)this + 0x10) + 0x34),0,0x200000,0,0,0,0,
                            0x1d,&DAT_00000002);
      FUN_00435790(this_00,(int)pvVar3);
      puVar4 = FUN_004357b0(this_00,&param_1);
      local_4 = 4;
      FUN_004f26d0(this_01,puVar4);
      local_4 = 0xffffffff;
      FUN_00619730();
      FUN_005f58b0(this_00);
    }
    if ((*this_01 >> 0x18 < 0x90) || (0x97 < *this_01 >> 0x18)) {
      bVar1 = false;
    }
    else {
      bVar1 = true;
    }
    param_1 = (uint *)CONCAT31(param_1._1_3_,!bVar1);
    FUN_00619730();
    if ((char)param_1 != '\0') {
      uVar5 = 0x400000;
      pvVar3 = FUN_0041a340(*(void **)((int)this + 0xc),
                            (undefined4 *)(*(int *)((int)this + 0x10) + 0x34),0,0x400000,0,0,0,0,
                            0x1d,&DAT_00000002);
      FUN_00435790(this_00,(int)pvVar3);
      puVar4 = FUN_004357b0(this_00,&param_1);
      local_4 = 5;
      FUN_004f26d0(this_01,puVar4);
      local_4 = 0xffffffff;
      FUN_00619730();
      FUN_005f58b0(this_00);
    }
    if ((*this_01 >> 0x18 < 0x90) || (0x97 < *this_01 >> 0x18)) {
      bVar1 = false;
    }
    else {
      bVar1 = true;
    }
    param_1 = (uint *)CONCAT31(param_1._1_3_,!bVar1);
    FUN_00619730();
    if ((char)param_1 != '\0') {
      uVar5 = 0x800000;
      pvVar3 = FUN_0041a340(*(void **)((int)this + 0xc),
                            (undefined4 *)(*(int *)((int)this + 0x10) + 0x34),0,0x800000,0x200,0,0,0
                            ,0x1d,&DAT_00000002);
      FUN_00435790(this_00,(int)pvVar3);
      puVar4 = FUN_004357b0(this_00,&param_1);
      local_4 = 6;
      FUN_004f26d0(this_01,puVar4);
      local_4 = 0xffffffff;
      FUN_00619730();
      FUN_005f58b0(this_00);
    }
  }
  if ((*this_01 >> 0x18 < 0x90) || (0x97 < *this_01 >> 0x18)) {
    bVar1 = false;
  }
  else {
    bVar1 = true;
  }
  FUN_00619730();
  if (bVar1) {
    FUN_004ae480(this,this_01,param_2,uVar5);
  }
  ExceptionList = local_c;
  return;
}

