
void __thiscall FUN_0042d050(void *this,int param_1,undefined4 param_2,int param_3)

{
  int iVar1;
  int iVar2;
  int *piVar3;
  byte bVar4;
  void *pvVar5;
  undefined4 local_14 [2];
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0062c798;
  local_c = ExceptionList;
  if (param_3 == 0) {
    return;
  }
  ExceptionList = &local_c;
  iVar1 = FUN_00401060();
  FUN_005205a0(local_14);
  local_4 = 0;
  if (param_3 == 0xbeef) {
    FUN_0041d190(local_14);
  }
  else {
    if (param_3 != 0xdead) goto LAB_0042d148;
    FUN_0041d160(local_14,1);
  }
  for (iVar2 = thunk_FUN_005f5060((int)local_14); iVar2 != 0; iVar2 = *(int *)(iVar2 + 0x10)) {
    bVar4 = *(int *)(iVar2 + 0x1c) == 0;
    if (*(int *)(iVar2 + 0x20) != 0) {
      bVar4 = bVar4 | 2;
    }
    FUN_00442590(*(uint *)(iVar2 + 0x18),(void *)0x7,(uint)bVar4,param_1,param_2,this,
                 (-(uint)(iVar1 != 1) & 0xfe01) + 0x20000ff,0x2ffffff,0x2808080);
  }
  FUN_005ff440((int)this);
  pvVar5 = this;
  piVar3 = (int *)FUN_006037f0(7);
  FUN_00442380(param_1,param_2,piVar3,(int)this,pvVar5);
LAB_0042d148:
  local_4 = 0xffffffff;
  FUN_005205e0(local_14);
  ExceptionList = local_c;
  return;
}

