
void __thiscall FUN_0050b5a0(void *this,void *param_1)

{
  int iVar1;
  uint uVar2;
  uint uVar3;
  int iVar4;
  uint uVar5;
  int iVar6;
  
  iVar1 = 0;
  uVar5 = *(uint *)((int)this + 0x24) >> 6 & 3;
  if (uVar5 == 1) {
    iVar4 = 1;
  }
  else if (uVar5 == 2) {
    iVar4 = 2;
  }
  else {
    iVar4 = 0;
  }
  if ((iVar4 != 0) && (uVar3 = *(uint *)((int)this + 0x88), (uVar3 & 1) != 0)) {
    iVar6 = 1;
    uVar2 = uVar3 >> 2 & 1;
    uVar3 = uVar3 >> 0xb & 1;
    iVar1 = FUN_00507270(this,iVar4);
    iVar1 = FUN_00559fe0(uVar5,iVar1,uVar3,uVar2,iVar6);
  }
  FUN_0050a290(this,iVar1,param_1);
  return;
}

