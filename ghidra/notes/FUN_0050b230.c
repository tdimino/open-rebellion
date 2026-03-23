
void __thiscall FUN_0050b230(void *this,void *param_1)

{
  int iVar1;
  int iVar2;
  int iVar3;
  int iVar4;
  uint uVar5;
  int iVar6;
  uint uVar7;
  
  iVar1 = 0;
  uVar7 = *(uint *)((int)this + 0x24) >> 6 & 3;
  if (uVar7 == 1) {
    iVar6 = 1;
  }
  else if (uVar7 == 2) {
    iVar6 = 2;
  }
  else {
    iVar6 = 0;
  }
  if (iVar6 != 0) {
    uVar5 = *(uint *)((int)this + 0x88) >> 0xb & 1;
    iVar1 = FUN_00509140(this,uVar7,1);
    iVar2 = FUN_00509020(this,uVar7,1);
    iVar3 = FUN_00508fb0(this,uVar7,1);
    iVar4 = FUN_00509090(this,uVar7,1);
    iVar6 = FUN_00507270(this,iVar6);
    iVar1 = FUN_00559c40(uVar7,iVar6,uVar5,iVar4,iVar3,iVar2,iVar1);
  }
  FUN_00509ec0(this,iVar1,param_1);
  return;
}

