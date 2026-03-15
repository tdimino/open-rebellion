
undefined4 __fastcall FUN_004e2760(int param_1)

{
  uint *this;
  void *pvVar1;
  bool bVar2;
  void *pvVar3;
  undefined4 *puVar4;
  undefined4 local_18;
  undefined4 local_14;
  undefined4 local_10;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0063ddd0;
  local_c = ExceptionList;
  local_18 = 1;
  pvVar1 = *(void **)(param_1 + 0x2c);
  if (*(int *)((int)pvVar1 + 0x184) < *(int *)((int)pvVar1 + 0x188)) {
    ExceptionList = &local_c;
    pvVar3 = FUN_00419a00(pvVar1,0xe0,0,0,0,0,0,0x15,(void *)0x1);
    pvVar1 = (void *)(param_1 + 0x70);
    FUN_00435790(pvVar1,(int)pvVar3);
    pvVar3 = FUN_00419a00(*(void **)(param_1 + 0x2c),0xe0,0,0,0,0,0,0x11,(void *)0x1);
    FUN_00435790(pvVar1,(int)pvVar3);
    pvVar3 = FUN_00419a00(*(void **)(param_1 + 0x2c),0xe0,0,0,0,0,0,0x13,(void *)0x1);
    FUN_00435790(pvVar1,(int)pvVar3);
    puVar4 = (undefined4 *)FUN_004357b0(pvVar1,&local_14);
    this = (uint *)(param_1 + 0x48);
    local_4 = 0;
    FUN_004f26d0(this,puVar4);
    local_4 = 0xffffffff;
    FUN_00619730();
    FUN_005f58b0(pvVar1);
    pvVar3 = FUN_0041a340(*(void **)(param_1 + 0x2c),this,0,0,1,0x3e00000,0,0,6,(void *)0x1);
    FUN_00435790(pvVar1,(int)pvVar3);
    pvVar3 = FUN_0041a340(*(void **)(param_1 + 0x2c),this,0,0,1,0x3e00000,0,0,4,(void *)0x1);
    FUN_00435790(pvVar1,(int)pvVar3);
    pvVar3 = FUN_0041a340(*(void **)(param_1 + 0x2c),this,0,0,1,0x3e00000,0,0,5,(void *)0x1);
    FUN_00435790(pvVar1,(int)pvVar3);
    puVar4 = (undefined4 *)FUN_004357b0(pvVar1,&local_14);
    local_4 = 1;
    FUN_004f26d0(this,puVar4);
    local_4 = 0xffffffff;
    FUN_00619730();
    FUN_005f58b0(pvVar1);
    local_14 = 0x90;
    local_10 = 0x98;
    if ((*this >> 0x18 < 0x90) || (0x97 < *this >> 0x18)) {
      bVar2 = false;
    }
    else {
      bVar2 = true;
    }
    FUN_00619730();
    if (bVar2) {
      local_18 = 0;
    }
  }
  ExceptionList = local_c;
  return local_18;
}

