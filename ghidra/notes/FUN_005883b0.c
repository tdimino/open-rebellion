
bool __thiscall FUN_005883b0(void *this,uint param_1,int param_2,undefined4 *param_3)

{
  int *this_00;
  uint *puVar1;
  uint *puVar2;
  undefined4 uVar3;
  bool bVar4;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0064f908;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  *param_3 = 0;
  bVar4 = true;
  if (((*(uint *)(param_1 + 0x78) & 0xc) == 0) &&
     ((param_2 == 0 || ((*(uint *)(param_1 + 0x78) & 0x20) == 0)))) {
    this_00 = *(int **)(param_1 + 0x1c);
    bVar4 = this_00 != (int *)0x0;
    if (this_00 != (int *)0x0) {
      puVar1 = FUN_0050c640(this_00);
      local_4 = 0;
      puVar2 = FUN_004025b0(this_00,&param_1);
      if (*puVar2 == *puVar1) {
        uVar3 = *(undefined4 *)((int)this + 0x10);
      }
      else {
        uVar3 = *(undefined4 *)((int)this + 0x14);
      }
      *param_3 = uVar3;
      FUN_00619730();
      local_4 = 0xffffffff;
      FUN_00619730();
    }
  }
  ExceptionList = local_c;
  return bVar4;
}

