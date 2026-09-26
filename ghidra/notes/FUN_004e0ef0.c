
void __fastcall FUN_004e0ef0(int param_1)

{
  int iVar1;
  undefined4 uVar2;
  undefined4 *puVar3;
  undefined4 *puVar4;
  void *pvVar5;
  void *pvVar6;
  undefined1 local_14 [4];
  undefined1 *local_10;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0063db30;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  puVar3 = FUN_00419b80(*(void **)(param_1 + 0x2c),(undefined4 *)(param_1 + 0x50),0x400000,0,0,0,0,0
                        ,2);
  pvVar6 = (void *)(param_1 + 0x40);
  FUN_00435790(pvVar6,(int)puVar3);
  puVar4 = FUN_004357b0(pvVar6,local_14);
  puVar3 = (undefined4 *)(param_1 + 0x54);
  local_4 = 0;
  FUN_004f26d0(puVar3,puVar4);
  local_4 = 0xffffffff;
  FUN_00619730();
  FUN_005f58b0(pvVar6);
  pvVar5 = FUN_0041a340(*(void **)(param_1 + 0x2c),puVar3,0x20000,0,1,3,0,0,0x15,&DAT_00000002);
  FUN_00435790(pvVar6,(int)pvVar5);
  puVar4 = FUN_004357b0(pvVar6,local_14);
  local_4 = 1;
  FUN_004f26d0(puVar3,puVar4);
  local_4 = 0xffffffff;
  FUN_00619730();
  FUN_005f58b0(pvVar6);
  iVar1 = *(int *)(param_1 + 0x2c);
  local_10 = &stack0xffffffdc;
  FUN_004f26d0(&stack0xffffffdc,puVar3);
  pvVar6 = FUN_00403d30((void *)(iVar1 + 0x2c));
  if (pvVar6 == (void *)0x0) {
    *(uint *)(param_1 + 0x28) = *(uint *)(param_1 + 0x28) | 8;
    ExceptionList = local_c;
    return;
  }
  if ((*(uint *)((int)pvVar6 + 0x28) & 0x83e00000) != 0) {
    uVar2 = *(undefined4 *)((int)pvVar6 + 0x9c);
    *(undefined4 *)(param_1 + 0x74) = 1;
    *(undefined4 *)(param_1 + 0x60) = uVar2;
    *(undefined4 *)(param_1 + 0x6c) = 0x2000;
    *(undefined4 *)(param_1 + 0x70) = 0x2000;
    ExceptionList = local_c;
    return;
  }
  uVar2 = *(undefined4 *)((int)pvVar6 + 0x9c);
  *(undefined4 *)(param_1 + 0x6c) = 0x2000;
  *(undefined4 *)(param_1 + 0x60) = uVar2;
  *(undefined4 *)(param_1 + 0x70) = 1;
  *(undefined4 *)(param_1 + 0x74) = 2;
  ExceptionList = local_c;
  return;
}

