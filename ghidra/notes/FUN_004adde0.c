
void __thiscall FUN_004adde0(void *this,uint *param_1)

{
  int iVar1;
  bool bVar2;
  uint *this_00;
  uint uVar3;
  uint *puVar4;
  void *pvVar5;
  void *pvVar6;
  undefined4 *puVar7;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_006388b0;
  local_c = ExceptionList;
  uVar3 = *(uint *)(*(int *)((int)this + 0x10) + 0x40) >> 0x18;
  if ((uVar3 < 0x90) || (0x97 < uVar3)) {
    bVar2 = false;
  }
  else {
    bVar2 = true;
  }
  ExceptionList = &local_c;
  FUN_00619730();
  this_00 = param_1;
  if (bVar2) {
    puVar4 = (uint *)(*(int *)((int)this + 0x10) + 0x40);
    if (*param_1 != *puVar4) {
      iVar1 = *(int *)((int)this + 0xc);
      param_1 = (uint *)&stack0xffffffd8;
      FUN_004f26d0(&stack0xffffffd8,puVar4);
      pvVar5 = FUN_00403d30((void *)(iVar1 + 0x2c));
      if (((pvVar5 != (void *)0x0) && ((*(byte *)((int)pvVar5 + 0x30) & 1) != 0)) &&
         ((*(byte *)((int)pvVar5 + 0x28) & 3) == 0)) {
        FUN_004f26d0(this_00,(undefined4 *)(*(int *)((int)this + 0x10) + 0x40));
      }
    }
  }
  uVar3 = *this_00 >> 0x18;
  if (((uVar3 < 0x90) || (0x97 < uVar3)) ||
     (*this_00 == *(uint *)(*(int *)((int)this + 0x10) + 0x40))) {
    bVar2 = false;
  }
  else {
    bVar2 = true;
  }
  FUN_00619730();
  if (bVar2) {
    iVar1 = *(int *)((int)this + 0xc);
    param_1 = (uint *)&stack0xffffffd8;
    FUN_004f26d0(&stack0xffffffd8,this_00);
    pvVar5 = FUN_00403d30((void *)(iVar1 + 0x2c));
    if ((pvVar5 != (void *)0x0) &&
       (((*(byte *)((int)pvVar5 + 0x30) & 1) == 0 || ((*(byte *)((int)pvVar5 + 0x28) & 3) != 0)))) {
      FUN_004ece80(this_00);
    }
  }
  if ((*this_00 >> 0x18 < 0x90) || (0x97 < *this_00 >> 0x18)) {
    bVar2 = false;
  }
  else {
    bVar2 = true;
  }
  FUN_00619730();
  if (!bVar2) {
    pvVar6 = FUN_0041a460(*(void **)((int)this + 0xc),
                          (undefined4 *)(*(int *)((int)this + 0x10) + 0x34),0x80000000,0,1,3,0,0,
                          (void *)0x1);
    pvVar5 = (void *)((int)this + 0x18);
    FUN_00435790(pvVar5,(int)pvVar6);
    puVar7 = FUN_004357b0(pvVar5,&param_1);
    local_4 = 0;
    FUN_004f26d0(this_00,puVar7);
    local_4 = 0xffffffff;
    FUN_00619730();
    FUN_005f58b0(pvVar5);
  }
  if ((*this_00 >> 0x18 < 0x90) || (0x97 < *this_00 >> 0x18)) {
    bVar2 = false;
  }
  else {
    bVar2 = true;
  }
  FUN_00619730();
  if (!bVar2) {
    pvVar6 = FUN_0041a460(*(void **)((int)this + 0xc),
                          (undefined4 *)(*(int *)((int)this + 0x10) + 0x34),0x3e00000,0,1,3,0,0,
                          (void *)0x1);
    pvVar5 = (void *)((int)this + 0x18);
    FUN_00435790(pvVar5,(int)pvVar6);
    puVar7 = FUN_004357b0(pvVar5,&param_1);
    local_4 = 1;
    FUN_004f26d0(this_00,puVar7);
    local_4 = 0xffffffff;
    FUN_00619730();
    FUN_005f58b0(pvVar5);
  }
  if ((*this_00 >> 0x18 < 0x90) || (0x97 < *this_00 >> 0x18)) {
    bVar2 = false;
  }
  else {
    bVar2 = true;
  }
  FUN_00619730();
  if (!bVar2) {
    pvVar6 = FUN_0041a460(*(void **)((int)this + 0xc),
                          (undefined4 *)(*(int *)((int)this + 0x10) + 0x34),0,0,1,3,0,0,(void *)0x1)
    ;
    pvVar5 = (void *)((int)this + 0x18);
    FUN_00435790(pvVar5,(int)pvVar6);
    puVar7 = FUN_004357b0(pvVar5,&param_1);
    local_4 = 2;
    FUN_004f26d0(this_00,puVar7);
    local_4 = 0xffffffff;
    FUN_00619730();
    FUN_005f58b0(pvVar5);
  }
  if ((*this_00 >> 0x18 < 0x90) || (0x97 < *this_00 >> 0x18)) {
    bVar2 = false;
  }
  else {
    bVar2 = true;
  }
  FUN_00619730();
  if (!bVar2) {
    puVar7 = FUN_0041b2b0(*(void **)((int)this + 0xc),&param_1,
                          (uint *)(*(int *)((int)this + 0x10) + 0x34),0,1,0,3,0,0);
    local_4 = 3;
    FUN_004f26d0(this_00,puVar7);
    local_4 = 0xffffffff;
    FUN_00619730();
  }
  ExceptionList = local_c;
  return;
}

