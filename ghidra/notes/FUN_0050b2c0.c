
void __thiscall FUN_0050b2c0(void *this,void *param_1)

{
  int iVar1;
  uint uVar2;
  int iVar3;
  
  iVar1 = 100;
  uVar2 = *(uint *)((int)this + 0x24) >> 6 & 3;
  if (uVar2 == 1) {
    iVar3 = 1;
  }
  else if (uVar2 == 2) {
    iVar3 = 2;
  }
  else {
    iVar3 = 0;
  }
  if (iVar3 != 0) {
    iVar1 = FUN_00507270(this,iVar3);
    iVar1 = FUN_00559c10(iVar1);
  }
  FUN_00509f40(this,iVar1,param_1);
  return;
}

