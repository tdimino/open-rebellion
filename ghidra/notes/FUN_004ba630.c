
void __thiscall FUN_004ba630(void *this,uint *param_1,undefined4 *param_2)

{
  void *this_00;
  bool bVar1;
  uint *this_01;
  uint uVar2;
  void *pvVar3;
  undefined4 *puVar4;
  uint local_1c;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  this_01 = param_1;
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00639990;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  FUN_004ece80(param_1);
  FUN_004ece80(param_2);
  local_1c = 0x80;
  uVar2 = *(uint *)(*(int *)((int)this + 0x10) + 0x34) >> 0x18;
  if ((uVar2 < 0x80) || (0x8f < uVar2)) {
    bVar1 = false;
  }
  else {
    bVar1 = true;
  }
  FUN_00619730();
  puVar4 = param_2;
  if (bVar1) {
    local_1c = 0xc00;
    pvVar3 = FUN_0041a340(*(void **)((int)this + 0xc),
                          (undefined4 *)(*(int *)((int)this + 0x10) + 0x34),0,0x400,0,0,0,0,0x1d,
                          &DAT_00000002);
    this_00 = (void *)((int)this + 0x18);
    FUN_00435790(this_00,(int)pvVar3);
    puVar4 = (undefined4 *)FUN_004357b0(this_00,&param_1);
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
    puVar4 = (undefined4 *)0x4;
    if ((char)param_1 != '\0') {
      local_1c = 0x800;
      pvVar3 = FUN_0041a340(*(void **)((int)this + 0xc),
                            (undefined4 *)(*(int *)((int)this + 0x10) + 0x34),0,0x800,0,0,0,0,0x1d,
                            &DAT_00000002);
      FUN_00435790(this_00,(int)pvVar3);
      puVar4 = (undefined4 *)FUN_004357b0(this_00,&param_1);
      local_4 = 1;
      FUN_004f26d0(this_01,puVar4);
      local_4 = 0xffffffff;
      FUN_00619730();
      FUN_005f58b0(this_00);
      puVar4 = (undefined4 *)0x4;
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
    FUN_004af1c0(this,this_01,param_2,local_1c,(uint)puVar4);
  }
  ExceptionList = local_c;
  return;
}

