
int __thiscall FUN_00528100(void *this,int param_1)

{
  int iVar1;
  int *piVar2;
  int iVar3;
  undefined4 local_2c [8];
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00644528;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  iVar1 = FUN_0052bed0((int)this);
  iVar3 = 0;
  if (iVar1 != 0) {
    FUN_0052cda0(local_2c,iVar1);
    local_4 = 0;
    FUN_00513120((int)local_2c);
    do {
      iVar1 = FUN_0052bed0((int)local_2c);
      if (iVar1 == 0) break;
      piVar2 = (int *)FUN_0052bed0((int)local_2c);
      iVar1 = FUN_0052bde0(this,piVar2);
      if (iVar1 == 0) {
LAB_00528185:
        iVar3 = 0;
      }
      else {
        iVar1 = FUN_0052bed0((int)local_2c);
        if (((*(uint *)(iVar1 + 0x60) & 1) != 0) && (param_1 == 0)) goto LAB_00528185;
        iVar3 = 1;
      }
      FUN_005130d0((int)local_2c);
    } while (iVar3 == 0);
    local_4 = 0xffffffff;
    FUN_0052cec0(local_2c);
  }
  ExceptionList = local_c;
  return iVar3;
}

