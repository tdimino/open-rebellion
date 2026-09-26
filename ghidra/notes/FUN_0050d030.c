
void __thiscall FUN_0050d030(void *this,void *param_1)

{
  int iVar1;
  int iVar2;
  int iVar3;
  int iVar4;
  int iVar5;
  uint uVar6;
  uint uVar7;
  undefined4 *puVar8;
  undefined4 *puVar9;
  int local_c;
  undefined4 local_8;
  undefined4 local_4;
  
  uVar7 = *(uint *)((int)this + 0x24) >> 6 & 3;
  if (uVar7 == 1) {
    local_c = 1;
  }
  else {
    if (uVar7 != 2) {
      return;
    }
    local_c = 2;
  }
  FUN_0050aac0(this,1,param_1);
  FUN_0050aac0(this,0,param_1);
  local_8 = 0;
  local_4 = 0;
  puVar9 = &local_4;
  puVar8 = &local_8;
  uVar6 = *(uint *)((int)this + 0x88) >> 0xb & 1;
  iVar1 = FUN_00509b20((int)this);
  iVar2 = FUN_00509b10((int)this);
  iVar3 = FUN_005091f0(this);
  iVar4 = FUN_00509020(this,uVar7,1);
  iVar5 = FUN_00507270(this,local_c);
  FUN_00559ce0(uVar7,iVar5,uVar6,iVar4,iVar3,iVar2,iVar1,puVar8,puVar9);
  FUN_0050d150(this,local_8,uVar7,param_1);
  FUN_0050d150(this,local_4,uVar7,param_1);
  iVar1 = FUN_00509b30((int)this);
  FUN_0050c9f0(this,iVar1,uVar7,param_1);
  return;
}

