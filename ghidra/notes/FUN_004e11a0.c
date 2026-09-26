
void __fastcall FUN_004e11a0(int param_1)

{
  undefined4 *this;
  int iVar1;
  void *pvVar2;
  undefined4 *puVar3;
  void *pvVar4;
  undefined1 local_14 [4];
  undefined1 *local_10;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0063db70;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  pvVar2 = FUN_00419a00(*(void **)(param_1 + 0x2c),0x40000000,0,0,0,0,0,0x1e,&DAT_00000002);
  pvVar4 = (void *)(param_1 + 0x40);
  FUN_00435790(pvVar4,(int)pvVar2);
  puVar3 = FUN_004357b0(pvVar4,local_14);
  this = (undefined4 *)(param_1 + 0x54);
  local_4 = 0;
  FUN_004f26d0(this,puVar3);
  local_4 = 0xffffffff;
  FUN_00619730();
  FUN_005f58b0(pvVar4);
  pvVar2 = FUN_0041a340(*(void **)(param_1 + 0x2c),this,0x2000,0,1,3,0,0,0x15,&DAT_00000002);
  FUN_00435790(pvVar4,(int)pvVar2);
  puVar3 = FUN_004357b0(pvVar4,local_14);
  local_4 = 1;
  FUN_004f26d0(this,puVar3);
  local_4 = 0xffffffff;
  FUN_00619730();
  FUN_005f58b0(pvVar4);
  iVar1 = *(int *)(param_1 + 0x2c);
  local_10 = &stack0xffffffdc;
  FUN_004f26d0(&stack0xffffffdc,this);
  pvVar4 = FUN_00403d30((void *)(iVar1 + 0x2c));
  if (pvVar4 != (void *)0x0) {
    *(undefined4 *)(param_1 + 0x60) = *(undefined4 *)((int)pvVar4 + 0x9c);
    *(undefined4 *)(param_1 + 0x6c) = 0x2000;
    *(undefined4 *)(param_1 + 0x70) = 0x2000;
    *(undefined4 *)(param_1 + 0x74) = 1;
    ExceptionList = local_c;
    return;
  }
  *(uint *)(param_1 + 0x28) = *(uint *)(param_1 + 0x28) | 2;
  ExceptionList = local_c;
  return;
}

