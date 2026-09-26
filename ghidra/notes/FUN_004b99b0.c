
void __thiscall FUN_004b99b0(void *this,uint *param_1,undefined4 *param_2)

{
  void *pvVar1;
  bool bVar2;
  uint *this_00;
  uint uVar3;
  void *pvVar4;
  undefined4 *puVar5;
  uint local_18;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  this_00 = param_1;
  local_4 = 0xffffffff;
  puStack_8 = &LAB_006398c8;
  local_c = ExceptionList;
  bVar2 = false;
  local_18 = 0;
  ExceptionList = &local_c;
  FUN_004ece80(param_1);
  FUN_004ece80(param_2);
  uVar3 = *(uint *)(*(int *)((int)this + 0x10) + 0x34) >> 0x18;
  if ((0x7f < uVar3) && (uVar3 < 0x90)) {
    bVar2 = true;
  }
  FUN_00619730();
  if (bVar2) {
    if ((*this_00 >> 0x18 < 0x90) || (0x97 < *this_00 >> 0x18)) {
      bVar2 = false;
    }
    else {
      bVar2 = true;
    }
    FUN_00619730();
    if (!bVar2) {
      local_18 = 0x400000;
      pvVar4 = FUN_0041a340(*(void **)((int)this + 0xc),
                            (undefined4 *)(*(int *)((int)this + 0x10) + 0x34),0,0x400000,0,0,0,0,
                            0x1d,&DAT_00000002);
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
    param_1 = (uint *)CONCAT31(param_1._1_3_,!bVar2);
    FUN_00619730();
    if ((char)param_1 != '\0') {
      local_18 = 0x800000;
      pvVar4 = FUN_0041a340(*(void **)((int)this + 0xc),
                            (undefined4 *)(*(int *)((int)this + 0x10) + 0x34),0,0x800000,0x200,0,0,0
                            ,0x1d,&DAT_00000002);
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
    param_1 = (uint *)CONCAT31(param_1._1_3_,!bVar2);
    FUN_00619730();
    if ((char)param_1 != '\0') {
      local_18 = 0x20000;
      pvVar4 = FUN_0041a340(*(void **)((int)this + 0xc),
                            (undefined4 *)(*(int *)((int)this + 0x10) + 0x34),0,0x20000,0,0,0,0,0x1d
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
  }
  if ((*this_00 >> 0x18 < 0x90) || (0x97 < *this_00 >> 0x18)) {
    bVar2 = false;
  }
  else {
    bVar2 = true;
  }
  FUN_00619730();
  if (bVar2) {
    FUN_004ae480(this,this_00,param_2,local_18);
  }
  ExceptionList = local_c;
  return;
}

