
uint __thiscall FUN_005018c0(void *this,int param_1,undefined4 param_2,void *param_3)

{
  uint uVar1;
  bool bVar2;
  int iVar3;
  int iVar4;
  undefined3 extraout_var;
  uint uVar5;
  undefined3 extraout_var_00;
  undefined3 extraout_var_01;
  uint uVar6;
  void *pvStack_4;
  
  pvStack_4 = this;
  iVar3 = (**(code **)(*(int *)this + 0x274))();
  iVar4 = (**(code **)(*(int *)this + 0x248))();
  bVar2 = FUN_005585e0(iVar4 - iVar3,param_1,&pvStack_4);
  uVar6 = 0;
  if (CONCAT31(extraout_var,bVar2) != 0) {
    if (((int)pvStack_4 < 1) || (7 < (int)pvStack_4)) {
      uVar6 = 0;
    }
    else {
      uVar6 = 1;
    }
  }
  if (uVar6 != 0) {
    switch(pvStack_4) {
    case (void *)0x2:
      uVar1 = *(uint *)((int)this + 100);
      uVar5 = FUN_00500670();
      if (uVar5 != (uVar1 & 0xf)) {
        iVar3 = FUN_00501510(this,(*(uint *)((int)this + 100) & 0xf) + 1,param_3);
        return (uint)(iVar3 != 0);
      }
      break;
    case (void *)0x3:
      uVar1 = *(uint *)((int)this + 100);
      uVar5 = FUN_00500670();
      if (uVar5 != (uVar1 >> 4 & 0xf)) {
        iVar3 = FUN_005015a0(this,(*(uint *)((int)this + 100) >> 4 & 0xf) + 1,param_3);
        return (uint)(iVar3 != 0);
      }
      break;
    case (void *)0x4:
      uVar1 = *(uint *)((int)this + 100);
      uVar5 = FUN_00500670();
      if (uVar5 != (uVar1 >> 8 & 0xf)) {
        iVar3 = FUN_00501640(this,(*(uint *)((int)this + 100) >> 8 & 0xf) + 1,param_3);
        return (uint)(iVar3 != 0);
      }
      break;
    case (void *)0x5:
      uVar1 = *(uint *)((int)this + 100);
      uVar5 = FUN_00500670();
      if (uVar5 != (uVar1 >> 0xc & 0xf)) {
        iVar3 = FUN_005016e0(this,(*(uint *)((int)this + 100) >> 0xc & 0xf) + 1,param_3);
        return (uint)(iVar3 != 0);
      }
      break;
    case (void *)0x6:
      uVar1 = *(uint *)((int)this + 100);
      bVar2 = FUN_00500680(this);
      if (CONCAT31(extraout_var_00,bVar2) != (uVar1 >> 0x10 & 0xf)) {
        iVar3 = FUN_00501780(this,(*(uint *)((int)this + 100) >> 0x10 & 0xf) + 1,param_3);
        return (uint)(iVar3 != 0);
      }
      uVar1 = *(uint *)((int)this + 100);
      bVar2 = FUN_00500690(this);
      if (CONCAT31(extraout_var_01,bVar2) != (uVar1 >> 0x14 & 0xf)) {
        iVar3 = FUN_00501820(this,(*(uint *)((int)this + 100) >> 0x14 & 0xf) + 1,param_3);
        return (uint)(iVar3 != 0);
      }
      break;
    case (void *)0x7:
      iVar3 = (**(code **)(*(int *)this + 0xac))(param_2,param_3);
      uVar6 = (uint)(iVar3 != 0);
    }
  }
  return uVar6;
}

