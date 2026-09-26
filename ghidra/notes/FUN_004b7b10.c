
void __thiscall FUN_004b7b10(void *this,uint *param_1,undefined4 *param_2)

{
  int iVar1;
  bool bVar2;
  uint *this_00;
  uint uVar3;
  void *pvVar4;
  void *pvVar5;
  undefined4 *puVar6;
  uint uVar7;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  this_00 = param_1;
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00639688;
  local_c = ExceptionList;
  uVar7 = 0;
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
    iVar1 = *(int *)((int)this + 0xc);
    param_1 = (uint *)&stack0xffffffd8;
    FUN_004f26d0(&stack0xffffffd8,(undefined4 *)((int)this + 0x5c));
    pvVar4 = FUN_00403d30((void *)(iVar1 + 0x2c));
    if (((pvVar4 != (void *)0x0) && ((*(uint *)((int)pvVar4 + 0x30) & 1) == 0)) &&
       ((*(uint *)((int)pvVar4 + 0x30) & 0x100) != 0)) {
      FUN_004f26d0(this_00,(undefined4 *)((int)this + 0x5c));
    }
    uVar7 = 0x40000;
    pvVar5 = FUN_0041a340(*(void **)((int)this + 0xc),
                          (undefined4 *)(*(int *)((int)this + 0x10) + 0x34),0,0x40000,0x100,0,0,0,
                          0x1d,&DAT_00000002);
    pvVar4 = (void *)((int)this + 0x18);
    FUN_00435790(pvVar4,(int)pvVar5);
    puVar6 = FUN_004357b0(pvVar4,&param_1);
    local_4 = 0;
    FUN_004f26d0(this_00,puVar6);
    local_4 = 0xffffffff;
    FUN_00619730();
    FUN_005f58b0(pvVar4);
    if ((*this_00 >> 0x18 < 0x90) || (0x97 < *this_00 >> 0x18)) {
      bVar2 = false;
    }
    else {
      bVar2 = true;
    }
    param_1 = (uint *)CONCAT31(param_1._1_3_,!bVar2);
    FUN_00619730();
    if ((char)param_1 != '\0') {
      uVar7 = 0x80000;
      pvVar5 = FUN_0041a340(*(void **)((int)this + 0xc),
                            (undefined4 *)(*(int *)((int)this + 0x10) + 0x34),0,0x80000,0,0,0,0,0x1d
                            ,&DAT_00000002);
      FUN_00435790(pvVar4,(int)pvVar5);
      puVar6 = FUN_004357b0(pvVar4,&param_1);
      local_4 = 1;
      FUN_004f26d0(this_00,puVar6);
      local_4 = 0xffffffff;
      FUN_00619730();
      FUN_005f58b0(pvVar4);
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
      uVar7 = 0x100000;
      pvVar5 = FUN_0041a340(*(void **)((int)this + 0xc),
                            (undefined4 *)(*(int *)((int)this + 0x10) + 0x34),0,0x100000,0,0,0,0,
                            0x1d,&DAT_00000002);
      FUN_00435790(pvVar4,(int)pvVar5);
      puVar6 = FUN_004357b0(pvVar4,&param_1);
      local_4 = 2;
      FUN_004f26d0(this_00,puVar6);
      local_4 = 0xffffffff;
      FUN_00619730();
      FUN_005f58b0(pvVar4);
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
      uVar7 = 0x40000;
      pvVar5 = FUN_0041a340(*(void **)((int)this + 0xc),
                            (undefined4 *)(*(int *)((int)this + 0x10) + 0x34),0,0x40000,0,0,0,0,0x1d
                            ,&DAT_00000002);
      FUN_00435790(pvVar4,(int)pvVar5);
      puVar6 = FUN_004357b0(pvVar4,&param_1);
      local_4 = 3;
      FUN_004f26d0(this_00,puVar6);
      local_4 = 0xffffffff;
      FUN_00619730();
      FUN_005f58b0(pvVar4);
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
      uVar7 = 0x200000;
      pvVar5 = FUN_0041a340(*(void **)((int)this + 0xc),
                            (undefined4 *)(*(int *)((int)this + 0x10) + 0x34),0,0x200000,0,0,0,0,
                            0x1d,&DAT_00000002);
      FUN_00435790(pvVar4,(int)pvVar5);
      puVar6 = FUN_004357b0(pvVar4,&param_1);
      local_4 = 4;
      FUN_004f26d0(this_00,puVar6);
      local_4 = 0xffffffff;
      FUN_00619730();
      FUN_005f58b0(pvVar4);
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
      uVar7 = 0x400000;
      pvVar5 = FUN_0041a340(*(void **)((int)this + 0xc),
                            (undefined4 *)(*(int *)((int)this + 0x10) + 0x34),0,0x400000,0,0,0,0,
                            0x1d,&DAT_00000002);
      FUN_00435790(pvVar4,(int)pvVar5);
      puVar6 = FUN_004357b0(pvVar4,&param_1);
      local_4 = 5;
      FUN_004f26d0(this_00,puVar6);
      local_4 = 0xffffffff;
      FUN_00619730();
      FUN_005f58b0(pvVar4);
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
      uVar7 = 0x800000;
      pvVar5 = FUN_0041a340(*(void **)((int)this + 0xc),
                            (undefined4 *)(*(int *)((int)this + 0x10) + 0x34),0,0x800000,0x200,0,0,0
                            ,0x1d,&DAT_00000002);
      FUN_00435790(pvVar4,(int)pvVar5);
      puVar6 = FUN_004357b0(pvVar4,&param_1);
      local_4 = 6;
      FUN_004f26d0(this_00,puVar6);
      local_4 = 0xffffffff;
      FUN_00619730();
      FUN_005f58b0(pvVar4);
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
    FUN_004ae480(this,this_00,param_2,uVar7);
  }
  ExceptionList = local_c;
  return;
}

