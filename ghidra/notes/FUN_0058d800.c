
uint __thiscall FUN_0058d800(void *this,int param_1,int param_2,void *param_3)

{
  int iVar1;
  uint uVar2;
  undefined4 uVar3;
  uint uVar4;
  bool bVar5;
  undefined4 local_3c;
  undefined4 local_38;
  undefined4 local_34;
  undefined4 local_30;
  undefined4 local_2c [7];
  int local_10;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00650048;
  local_c = ExceptionList;
  uVar3 = 0;
  bVar5 = true;
  local_3c = 0;
  local_38 = 0;
  local_34 = 0;
  local_30 = 0;
  switch(*(undefined4 *)(param_2 + 0xc)) {
  case 0x220:
    uVar3 = 1;
    local_34 = 1;
    break;
  case 0x221:
    local_3c = 1;
    break;
  case 0x222:
    uVar3 = 1;
    local_3c = 1;
    local_38 = 1;
    break;
  case 0x223:
    uVar3 = 1;
    local_3c = 1;
    local_38 = 1;
    local_30 = 1;
    break;
  default:
    bVar5 = false;
  }
  uVar4 = 0;
  uVar2 = 0;
  if (bVar5) {
    ExceptionList = &local_c;
    FUN_004ffe70(local_2c,param_1,1);
    local_4 = 0;
    FUN_00513120((int)local_2c);
    bVar5 = true;
    do {
      if (local_10 == 0) break;
      iVar1 = FUN_0052bed0((int)local_2c);
      if ((*(uint *)(iVar1 + 0x58) >> 6 & 1) != 0) {
        if (uVar4 == 0) {
          iVar1 = FUN_0052bed0((int)local_2c);
          uVar4 = *(uint *)(iVar1 + 0x24) >> 6 & 3;
        }
        else {
          iVar1 = FUN_0052bed0((int)local_2c);
          bVar5 = uVar4 == (*(uint *)(iVar1 + 0x24) >> 6 & 3);
        }
      }
      FUN_005130d0((int)local_2c);
    } while (bVar5);
    local_4 = 0xffffffff;
    FUN_004fff70(local_2c);
    uVar2 = 0;
    if ((bVar5) && (uVar2 = 0, uVar4 != 0)) {
      *(int *)((int)this + 0x10) = param_1;
      *(uint *)((int)this + 0x14) = uVar4;
      *(uint *)((int)this + 0x18) = *(uint *)(param_1 + 0x24) >> 6 & 3;
      *(undefined4 *)((int)this + 0x34) = uVar3;
      *(undefined4 *)((int)this + 0x38) = local_3c;
      *(undefined4 *)((int)this + 0x3c) = local_38;
      *(undefined4 *)((int)this + 0x40) = local_34;
      *(undefined4 *)((int)this + 0x44) = local_30;
      uVar2 = FUN_0058d770(this,param_3);
    }
  }
  ExceptionList = local_c;
  return uVar2;
}

