
undefined4 __fastcall FUN_004d71e0(int param_1)

{
  uint *this;
  int iVar1;
  bool bVar2;
  void *pvVar3;
  undefined4 *puVar4;
  void *pvVar5;
  undefined1 *local_1c;
  undefined4 local_18;
  undefined4 local_14;
  undefined4 local_10;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0063c8f0;
  local_c = ExceptionList;
  local_18 = 0;
  ExceptionList = &local_c;
  pvVar3 = FUN_00419a00(*(void **)(param_1 + 0x2c),0x10000,0,0,0,0,0,0x1d,(void *)0x1);
  pvVar5 = (void *)(param_1 + 0x40);
  FUN_00435790(pvVar5,(int)pvVar3);
  puVar4 = (undefined4 *)FUN_004357b0(pvVar5,&local_1c);
  local_4 = 0;
  this = (uint *)(param_1 + 0x54);
  FUN_004f26d0(this,puVar4);
  local_4 = 0xffffffff;
  FUN_00619730();
  FUN_005f58b0(pvVar5);
  pvVar3 = FUN_0041a340(*(void **)(param_1 + 0x2c),this,0,0,1,0,0,0,0x25,(void *)0x1);
  FUN_00435790(pvVar5,(int)pvVar3);
  puVar4 = (undefined4 *)FUN_004357b0(pvVar5,&local_1c);
  local_4 = 1;
  FUN_004f26d0(this,puVar4);
  local_4 = 0xffffffff;
  FUN_00619730();
  FUN_005f58b0(pvVar5);
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
    iVar1 = *(int *)(param_1 + 0x2c);
    local_1c = &stack0xffffffd4;
    FUN_004f26d0(&stack0xffffffd4,this);
    pvVar5 = FUN_00403d30((void *)(iVar1 + 0x2c));
    if ((pvVar5 != (void *)0x0) && (0 < *(int *)((int)pvVar5 + 0xdc))) {
      ExceptionList = local_c;
      return 1;
    }
  }
  ExceptionList = local_c;
  return local_18;
}

