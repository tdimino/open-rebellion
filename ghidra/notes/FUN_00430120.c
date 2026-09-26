
void __thiscall FUN_00430120(void *this,uint *param_1,uint *param_2)

{
  bool bVar1;
  void *pvVar2;
  int *piVar3;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0062cbdb;
  local_c = ExceptionList;
  if ((*param_1 >> 0x18 < 8) || (0xf < *param_1 >> 0x18)) {
    bVar1 = false;
  }
  else {
    bVar1 = true;
  }
  ExceptionList = &local_c;
  FUN_00619730();
  if (!bVar1) {
    ExceptionList = local_c;
    return;
  }
  FUN_004f26d0(&stack0xffffffdc,param_1);
  pvVar2 = FUN_00419e40(this);
  if (pvVar2 == (void *)0x0) {
    piVar3 = FUN_004f36a0(*(int *)((int)this + 8),param_1);
    if (piVar3 == (int *)0x0) {
      ExceptionList = local_c;
      return;
    }
    pvVar2 = (void *)FUN_00618b70(0xcc);
    local_4 = 0;
    if (pvVar2 == (void *)0x0) {
      pvVar2 = (void *)0x0;
    }
    else {
      pvVar2 = FUN_00432e40(pvVar2,(undefined4 *)((int)this + 8),piVar3,
                            *(undefined4 *)((int)this + 0x10),*(undefined4 *)((int)this + 0xc));
    }
    local_4 = 0xffffffff;
    if (pvVar2 == (void *)0x0) {
      ExceptionList = local_c;
      return;
    }
    FUN_00433600((int)pvVar2);
    FUN_0042ff30(this,pvVar2);
  }
  FUN_00433320(pvVar2,param_2);
  ExceptionList = local_c;
  return;
}

