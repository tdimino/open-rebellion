
void * __thiscall
FUN_005ff910(void *this,undefined4 param_1,char *param_2,uint param_3,undefined4 param_4,
            undefined4 param_5,int param_6,int param_7,undefined4 param_8,char *param_9,
            undefined4 param_10,undefined4 param_11,int param_12,undefined4 param_13,
            undefined4 param_14,uint param_15)

{
  char cVar1;
  char *pcVar2;
  int iVar3;
  uint uVar4;
  uint uVar5;
  char *pcVar6;
  int local_18;
  int local_14;
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00655fdc;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  FUN_005f5c10(this);
  local_4 = 0;
  FUN_00604490((undefined4 *)((int)this + 0x6c));
  local_4._0_1_ = 1;
  FUN_0060f020((undefined4 *)((int)this + 0x74));
  local_4._0_1_ = 2;
  FUN_0060f020((undefined4 *)((int)this + 0x88));
  *(undefined ***)this = &PTR_FUN_0066df18;
  local_4 = CONCAT31(local_4._1_3_,3);
  local_14 = 0;
  local_18 = 0;
  if ((DAT_006be5b4 != 0) && ((param_15 & 0x10) == 0)) {
    param_12 = DAT_006be5b4;
  }
  if (param_2 == (char *)0x0) {
    *(undefined4 *)((int)this + 0x48) = 0;
  }
  else {
    uVar4 = 0xffffffff;
    pcVar2 = param_2;
    do {
      if (uVar4 == 0) break;
      uVar4 = uVar4 - 1;
      cVar1 = *pcVar2;
      pcVar2 = pcVar2 + 1;
    } while (cVar1 != '\0');
    pcVar2 = (char *)FUN_00618b70(~uVar4);
    uVar4 = 0xffffffff;
    *(char **)((int)this + 0x48) = pcVar2;
    do {
      pcVar6 = param_2;
      if (uVar4 == 0) break;
      uVar4 = uVar4 - 1;
      pcVar6 = param_2 + 1;
      cVar1 = *param_2;
      param_2 = pcVar6;
    } while (cVar1 != '\0');
    uVar4 = ~uVar4;
    pcVar6 = pcVar6 + -uVar4;
    for (uVar5 = uVar4 >> 2; uVar5 != 0; uVar5 = uVar5 - 1) {
      *(undefined4 *)pcVar2 = *(undefined4 *)pcVar6;
      pcVar6 = pcVar6 + 4;
      pcVar2 = pcVar2 + 4;
    }
    for (uVar4 = uVar4 & 3; uVar4 != 0; uVar4 = uVar4 - 1) {
      *pcVar2 = *pcVar6;
      pcVar6 = pcVar6 + 1;
      pcVar2 = pcVar2 + 1;
    }
  }
  if (param_9 == (char *)0x0) {
    *(undefined4 *)((int)this + 0x4c) = 0;
  }
  else {
    uVar4 = 0xffffffff;
    pcVar2 = param_9;
    do {
      if (uVar4 == 0) break;
      uVar4 = uVar4 - 1;
      cVar1 = *pcVar2;
      pcVar2 = pcVar2 + 1;
    } while (cVar1 != '\0');
    pcVar2 = (char *)FUN_00618b70(~uVar4);
    uVar4 = 0xffffffff;
    *(char **)((int)this + 0x4c) = pcVar2;
    do {
      pcVar6 = param_9;
      if (uVar4 == 0) break;
      uVar4 = uVar4 - 1;
      pcVar6 = param_9 + 1;
      cVar1 = *param_9;
      param_9 = pcVar6;
    } while (cVar1 != '\0');
    uVar4 = ~uVar4;
    pcVar6 = pcVar6 + -uVar4;
    for (uVar5 = uVar4 >> 2; uVar5 != 0; uVar5 = uVar5 - 1) {
      *(undefined4 *)pcVar2 = *(undefined4 *)pcVar6;
      pcVar6 = pcVar6 + 4;
      pcVar2 = pcVar2 + 4;
    }
    for (uVar4 = uVar4 & 3; uVar4 != 0; uVar4 = uVar4 - 1) {
      *pcVar2 = *pcVar6;
      pcVar6 = pcVar6 + 1;
      pcVar2 = pcVar2 + 1;
    }
  }
  *(undefined4 *)((int)this + 0x2c) = param_5;
  *(undefined4 *)((int)this + 0x20) = param_8;
  *(undefined4 *)((int)this + 0x28) = param_4;
  *(int *)((int)this + 0x30) = param_6;
  *(int *)((int)this + 0x34) = param_7;
  if ((param_3 & 0x800000) != 0) {
    local_14 = GetSystemMetrics(5);
    local_18 = GetSystemMetrics(6);
  }
  if ((param_3 & 0x400000) != 0) {
    iVar3 = GetSystemMetrics(7);
    local_14 = local_14 + iVar3;
    iVar3 = GetSystemMetrics(8);
    local_18 = local_18 + iVar3;
    if ((param_3 & 0x800000) != 0) {
      iVar3 = GetSystemMetrics(4);
      local_18 = local_18 + iVar3;
    }
  }
  *(uint *)((int)this + 0x50) = param_3;
  *(undefined4 *)((int)this + 0x58) = param_14;
  *(undefined4 *)((int)this + 0x5c) = param_10;
  *(undefined4 *)((int)this + 0x60) = param_11;
  *(int *)((int)this + 100) = param_12;
  *(int *)((int)this + 0x38) = param_6 - local_14;
  *(int *)((int)this + 0x3c) = param_7 - local_18;
  *(undefined4 *)((int)this + 0x18) = 0;
  *(undefined4 *)((int)this + 0x44) = 0;
  *(undefined4 *)((int)this + 0x68) = param_13;
  *(uint *)((int)this + 0x40) = param_15;
  *(undefined4 *)((int)this + 0x54) = 0;
  *(undefined4 *)((int)this + 0x1c) = param_1;
  *(undefined4 *)((int)this + 0x80) = 0;
  *(undefined4 *)((int)this + 0x7c) = 0;
  uVar4 = FUN_00603850();
  if (uVar4 != 0) {
    FUN_00600290(this,uVar4);
  }
  *(undefined4 *)((int)this + 0x84) = 0;
  *(undefined4 *)((int)this + 0x90) = 0;
  ExceptionList = pvStack_c;
  return this;
}

