
void __thiscall FUN_004bab30(void *this,uint *param_1,undefined4 *param_2)

{
  void *pvVar1;
  bool bVar2;
  uint *this_00;
  uint uVar3;
  void *pvVar4;
  undefined4 *puVar5;
  uint uVar6;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  this_00 = param_1;
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00639a10;
  local_c = ExceptionList;
  uVar6 = 0;
  ExceptionList = &local_c;
  FUN_004ece80(param_1);
  FUN_004ece80(param_2);
  uVar3 = *(uint *)(*(int *)((int)this + 0x10) + 0x34) >> 0x18;
  if ((uVar3 < 0x80) || (0x8f < uVar3)) {
    bVar2 = false;
  }
  else {
    bVar2 = true;
  }
  FUN_00619730();
  if (bVar2) {
    if (((*this_00 >> 0x18 < 0x90) || (0x97 < *this_00 >> 0x18)) && (*(int *)((int)this + 4) == 2))
    {
      bVar2 = true;
    }
    else {
      bVar2 = false;
    }
    FUN_00619730();
    if (bVar2) {
      uVar6 = 0x2000000;
      pvVar4 = FUN_0041a340(*(void **)((int)this + 0xc),
                            (undefined4 *)(*(int *)((int)this + 0x10) + 0x34),0,0x2000000,0x80000000
                            ,0,0,0,0x1d,&DAT_00000002);
      pvVar1 = (void *)((int)this + 0x18);
      FUN_00435790(pvVar1,(int)pvVar4);
      puVar5 = FUN_004357b0(pvVar1,&param_1);
      local_4 = 0;
      FUN_004f26d0(this_00,puVar5);
      local_4 = 0xffffffff;
      FUN_00619730();
      FUN_005f58b0(pvVar1);
    }
    if ((*this_00 >> 0x18 < 0x90) || (0x97 < *this_00 >> 0x18)) {
      bVar2 = false;
    }
    else {
      bVar2 = true;
    }
    FUN_00619730();
    if (!bVar2) {
      uVar6 = 0x20000;
      pvVar4 = FUN_0041a340(*(void **)((int)this + 0xc),
                            (undefined4 *)(*(int *)((int)this + 0x10) + 0x34),0,0x20000,0,0,0,0,0x1d
                            ,&DAT_00000002);
      pvVar1 = (void *)((int)this + 0x18);
      FUN_00435790(pvVar1,(int)pvVar4);
      puVar5 = FUN_004357b0(pvVar1,&param_1);
      local_4 = 1;
      FUN_004f26d0(this_00,puVar5);
      local_4 = 0xffffffff;
      FUN_00619730();
      FUN_005f58b0(pvVar1);
    }
    if ((*this_00 >> 0x18 < 0x90) || (0x97 < *this_00 >> 0x18)) {
      bVar2 = false;
    }
    else {
      bVar2 = true;
    }
    FUN_00619730();
    if (!bVar2) {
      uVar6 = 0x10000;
      pvVar4 = FUN_0041a340(*(void **)((int)this + 0xc),
                            (undefined4 *)(*(int *)((int)this + 0x10) + 0x34),0,0x10000,0,0,0,0,0x1d
                            ,&DAT_00000002);
      pvVar1 = (void *)((int)this + 0x18);
      FUN_00435790(pvVar1,(int)pvVar4);
      puVar5 = FUN_004357b0(pvVar1,&param_1);
      local_4 = 2;
      FUN_004f26d0(this_00,puVar5);
      local_4 = 0xffffffff;
      FUN_00619730();
      FUN_005f58b0(pvVar1);
    }
    if ((*this_00 >> 0x18 < 0x90) || (0x97 < *this_00 >> 0x18)) {
      bVar2 = false;
    }
    else {
      bVar2 = true;
    }
    FUN_00619730();
    if (!bVar2) {
      uVar6 = 0x200000;
      pvVar4 = FUN_0041a340(*(void **)((int)this + 0xc),
                            (undefined4 *)(*(int *)((int)this + 0x10) + 0x34),0,0x200000,0,0,0,0,
                            0x1d,&DAT_00000002);
      pvVar1 = (void *)((int)this + 0x18);
      FUN_00435790(pvVar1,(int)pvVar4);
      puVar5 = FUN_004357b0(pvVar1,&param_1);
      local_4 = 3;
      FUN_004f26d0(this_00,puVar5);
      local_4 = 0xffffffff;
      FUN_00619730();
      FUN_005f58b0(pvVar1);
    }
    if ((*this_00 >> 0x18 < 0x90) || (0x97 < *this_00 >> 0x18)) {
      bVar2 = false;
    }
    else {
      bVar2 = true;
    }
    FUN_00619730();
    if (!bVar2) {
      uVar6 = 0x800000;
      pvVar4 = FUN_0041a340(*(void **)((int)this + 0xc),
                            (undefined4 *)(*(int *)((int)this + 0x10) + 0x34),0,0x800000,0x200,0,0,0
                            ,0x1d,&DAT_00000002);
      pvVar1 = (void *)((int)this + 0x18);
      FUN_00435790(pvVar1,(int)pvVar4);
      puVar5 = FUN_004357b0(pvVar1,&param_1);
      local_4 = 4;
      FUN_004f26d0(this_00,puVar5);
      local_4 = 0xffffffff;
      FUN_00619730();
      FUN_005f58b0(pvVar1);
    }
    if ((*this_00 >> 0x18 < 0x90) || (0x97 < *this_00 >> 0x18)) {
      bVar2 = false;
    }
    else {
      bVar2 = true;
    }
    FUN_00619730();
    if (!bVar2) {
      uVar6 = 0x400000;
      pvVar4 = FUN_0041a340(*(void **)((int)this + 0xc),
                            (undefined4 *)(*(int *)((int)this + 0x10) + 0x34),0,0x400000,0,0,0,0,
                            0x1d,&DAT_00000002);
      pvVar1 = (void *)((int)this + 0x18);
      FUN_00435790(pvVar1,(int)pvVar4);
      puVar5 = FUN_004357b0(pvVar1,&param_1);
      local_4 = 5;
      FUN_004f26d0(this_00,puVar5);
      local_4 = 0xffffffff;
      FUN_00619730();
      FUN_005f58b0(pvVar1);
    }
  }
  if ((*this_00 >> 0x18 < 0x90) || (0x97 < *this_00 >> 0x18)) {
    bVar2 = false;
  }
  else {
    bVar2 = true;
  }
  FUN_00619730();
  if (bVar2) {
    FUN_004ae480(this,this_00,param_2,uVar6);
  }
  ExceptionList = local_c;
  return;
}

