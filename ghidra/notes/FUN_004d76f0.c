
int __fastcall FUN_004d76f0(int param_1)

{
  uint *this;
  bool bVar1;
  void *pvVar2;
  void *pvVar3;
  undefined4 *puVar4;
  int iVar5;
  int local_20;
  undefined1 *local_1c;
  undefined4 local_18;
  undefined1 *local_14;
  undefined4 local_10;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0063c9b0;
  local_c = ExceptionList;
  local_20 = 0;
  this = (uint *)(param_1 + 0x54);
  ExceptionList = &local_c;
  *this = *(uint *)(param_1 + 0x54) & 0xffffff | 0x90000000;
  local_1c = &stack0xffffffc8;
  *this = 0x90000109;
  iVar5 = *(int *)(param_1 + 0x2c);
  FUN_004f26d0(&stack0xffffffc8,this);
  pvVar2 = FUN_00403d30((void *)(iVar5 + 0x2c));
  if (((*(byte *)((int)pvVar2 + 0x30) & 1) == 0) || ((*(uint *)((int)pvVar2 + 0x28) & 2) != 0)) {
    FUN_004ece80(this);
  }
  else if ((*(uint *)((int)pvVar2 + 0x28) & 0x40000000) == 0) {
    if (*(int *)((int)pvVar2 + 100) < *(int *)(*(int *)(param_1 + 0x2c) + 0x36c)) {
      local_20 = 1;
    }
  }
  else {
    local_20 = 1;
  }
  if (local_20 == 0) {
    pvVar3 = FUN_00419a00(*(void **)(param_1 + 0x2c),0x4000,0,0,0,0,0,0x1c,(void *)0x1);
    pvVar2 = (void *)(param_1 + 0x40);
    FUN_00435790(pvVar2,(int)pvVar3);
    puVar4 = FUN_00419b80(*(void **)(param_1 + 0x2c),(undefined4 *)(param_1 + 0x50),0x4000,0,0,0,0,0
                          ,2);
    FUN_00435790(pvVar2,(int)puVar4);
    puVar4 = (undefined4 *)FUN_004357b0(pvVar2,&local_1c);
    local_4 = 0;
    FUN_004f26d0(this,puVar4);
    local_4 = 0xffffffff;
    FUN_00619730();
    FUN_005f58b0(pvVar2);
    pvVar3 = FUN_0041a340(*(void **)(param_1 + 0x2c),this,0x4000,0,1,0x4000003,0,0,0x25,
                          &DAT_00000002);
    FUN_00435790(pvVar2,(int)pvVar3);
    puVar4 = (undefined4 *)FUN_004357b0(pvVar2,&local_1c);
    local_4 = 1;
    FUN_004f26d0(this,puVar4);
    local_4 = 0xffffffff;
    FUN_00619730();
    FUN_005f58b0(pvVar2);
    local_14 = (undefined1 *)0x90;
    local_10 = 0x98;
    if ((*this >> 0x18 < 0x90) || (0x97 < *this >> 0x18)) {
      bVar1 = false;
    }
    else {
      bVar1 = true;
    }
    FUN_00619730();
    if (!bVar1) {
      *this = *this & 0xffffff | 0x90000000;
      local_14 = &stack0xffffffc8;
      *this = 0x90000109;
      local_1c = *(undefined1 **)(param_1 + 0x2c);
      FUN_004f26d0(&stack0xffffffc8,this);
      pvVar3 = FUN_00403d30(local_1c + 0x2c);
      if ((((*(byte *)((int)pvVar3 + 0x30) & 1) == 0) || (-1 < *(int *)((int)pvVar3 + 0xdc))) ||
         ((*(byte *)((int)pvVar3 + 0x28) & 2) != 0)) {
        FUN_004ece80(this);
      }
      else {
        local_20 = 1;
      }
    }
    local_14 = (undefined1 *)0x90;
    local_10 = 0x98;
    if ((*this >> 0x18 < 0x90) || (0x97 < *this >> 0x18)) {
      bVar1 = false;
    }
    else {
      bVar1 = true;
    }
    FUN_00619730();
    if (!bVar1) {
      puVar4 = FUN_00419b80(*(void **)(param_1 + 0x2c),(undefined4 *)(param_1 + 0x50),0x8000,0,0,0,0
                            ,0,2);
      FUN_00435790(pvVar2,(int)puVar4);
      puVar4 = (undefined4 *)FUN_004357b0(pvVar2,&local_1c);
      local_4 = 2;
      FUN_004f26d0(this,puVar4);
      local_4 = 0xffffffff;
      FUN_00619730();
      FUN_005f58b0(pvVar2);
      pvVar3 = FUN_0041a340(*(void **)(param_1 + 0x2c),this,0x4000,0,1,3,0,0,0x25,&DAT_00000002);
      FUN_00435790(pvVar2,(int)pvVar3);
      puVar4 = (undefined4 *)FUN_004357b0(pvVar2,&local_1c);
      local_4 = 3;
      FUN_004f26d0(this,puVar4);
      local_4 = 0xffffffff;
      FUN_00619730();
      FUN_005f58b0(pvVar2);
    }
    local_1c = (undefined1 *)0x90;
    local_18 = 0x98;
    if ((*this >> 0x18 < 0x90) || (0x97 < *this >> 0x18)) {
      bVar1 = false;
    }
    else {
      bVar1 = true;
    }
    FUN_00619730();
    if (bVar1) {
      iVar5 = *(int *)(param_1 + 0x2c);
      local_14 = &stack0xffffffc8;
      FUN_004f26d0(&stack0xffffffc8,this);
      pvVar2 = FUN_00403d30((void *)(iVar5 + 0x2c));
      if ((pvVar2 != (void *)0x0) && (*(int *)((int)pvVar2 + 0xdc) < 0)) {
        local_20 = 1;
      }
    }
    else {
      iVar5 = FUN_004da4d0(param_1);
      if ((0 < iVar5) && (*(int *)((int)*(void **)(param_1 + 0x2c) + 0x374) < 2)) {
        FUN_00418240(*(void **)(param_1 + 0x2c),4);
      }
    }
  }
  ExceptionList = local_c;
  return local_20;
}

