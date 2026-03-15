
void __thiscall FUN_004b51b0(void *this,uint *param_1,undefined4 *param_2)

{
  void *this_00;
  bool bVar1;
  uint *this_01;
  uint uVar2;
  void *pvVar3;
  undefined4 *puVar4;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  this_01 = param_1;
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00639390;
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
  if (bVar1) {
    pvVar3 = FUN_0041a3a0(*(void **)((int)this + 0xc),
                          (undefined4 *)(*(int *)((int)this + 0x10) + 0x34),0,0x200,4,0,0,0,
                          (void *)0x1);
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
    FUN_00619730();
    if (!bVar1) {
      pvVar3 = FUN_0041a460(*(void **)((int)this + 0xc),
                            (undefined4 *)(*(int *)((int)this + 0x10) + 0x34),0,0x200,0,0,0,0,
                            (void *)0x1);
      FUN_00435790(this_00,(int)pvVar3);
      puVar4 = (undefined4 *)FUN_004357b0(this_00,&param_1);
      local_4 = 1;
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
    FUN_004f26d0(param_2,this_01);
  }
  ExceptionList = local_c;
  return;
}

