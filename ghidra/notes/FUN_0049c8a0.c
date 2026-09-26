
void __thiscall FUN_0049c8a0(void *this,void *param_1)

{
  void *pvVar1;
  int iVar2;
  int iVar3;
  uint *unaff_retaddr;
  int local_18;
  undefined1 *puStack_14;
  void *pvStack_10;
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00637068;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  FUN_004ece30(&local_18);
  local_4 = 0;
  FUN_005f4d90(param_1,this);
  (**(code **)(*(int *)((int)this + 0x34) + 4))();
  FUN_0042dd00((void *)((int)this + 0x34),*(int *)this);
  FUN_005f4d90(param_1,(int)this + 0x78);
  FUN_005f4d90(param_1,(int)this + 0x7c);
  FUN_005f4990((void *)((int)this + 4),param_1);
  *(void **)((int)this + 0x1c) = (void *)((int)this + 0x34);
  (**(code **)(*(int *)((int)this + 8) + 0x10))();
  FUN_004c54b0((void *)((int)this + 0x20),param_1,(void *)((int)this + 8),this);
  iVar2 = (int)this + 0x80;
  iVar3 = 0x1e;
  do {
    FUN_005f4d90(param_1,iVar2);
    iVar2 = iVar2 + 4;
    iVar3 = iVar3 + -1;
  } while (iVar3 != 0);
  iVar2 = (int)this + 0xf8;
  iVar3 = 1;
  do {
    FUN_005f4d90(param_1,iVar2);
    iVar2 = iVar2 + 4;
    iVar3 = iVar3 + -1;
  } while (iVar3 != 0);
  FUN_005f4d90(param_1,&local_18);
  if (local_18 == 0) {
    *(undefined4 *)((int)this + 0x74) = 0;
  }
  else {
    FUN_004ece90(&stack0xffffffe4,param_1);
    puStack_14 = &stack0xffffffd0;
    FUN_004f26d0(&stack0xffffffd0,(undefined4 *)&stack0xffffffe4);
    pvVar1 = FUN_0049dca0((void *)((int)this + 0x20));
    *(void **)((int)this + 0x74) = pvVar1;
  }
  puStack_8 = (undefined1 *)0xffffffff;
  *unaff_retaddr = *unaff_retaddr | 0x80000000;
  FUN_00619730();
  ExceptionList = pvStack_10;
  return;
}

