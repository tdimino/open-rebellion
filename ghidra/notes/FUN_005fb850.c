
void * __thiscall
FUN_005fb850(void *this,undefined4 param_1,char *param_2,uint param_3,undefined4 param_4,
            undefined4 param_5,int param_6,int param_7,char *param_8,undefined4 param_9,
            undefined4 param_10,int param_11,undefined4 param_12,undefined4 param_13,uint param_14)

{
  char cVar1;
  char *pcVar2;
  uint uVar3;
  uint uVar4;
  char *pcVar5;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00655e48;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  FUN_005ff910(this,param_1,param_2,param_3,param_4,param_5,param_6,param_7,0,param_8,param_9,
               param_10,param_11,param_12,param_13,param_14);
  local_4 = 0;
  *(undefined ***)this = &PTR_FUN_0066de78;
  FUN_00601ea0(10);
  uVar3 = 0xffffffff;
  *(undefined4 *)((int)this + 0x9c) = 0;
  pcVar2 = param_2;
  do {
    if (uVar3 == 0) break;
    uVar3 = uVar3 - 1;
    cVar1 = *pcVar2;
    pcVar2 = pcVar2 + 1;
  } while (cVar1 != '\0');
  pcVar2 = (char *)FUN_00618b70(~uVar3);
  uVar3 = 0xffffffff;
  *(char **)((int)this + 0x94) = pcVar2;
  do {
    pcVar5 = param_2;
    if (uVar3 == 0) break;
    uVar3 = uVar3 - 1;
    pcVar5 = param_2 + 1;
    cVar1 = *param_2;
    param_2 = pcVar5;
  } while (cVar1 != '\0');
  uVar3 = ~uVar3;
  pcVar5 = pcVar5 + -uVar3;
  for (uVar4 = uVar3 >> 2; uVar4 != 0; uVar4 = uVar4 - 1) {
    *(undefined4 *)pcVar2 = *(undefined4 *)pcVar5;
    pcVar5 = pcVar5 + 4;
    pcVar2 = pcVar2 + 4;
  }
  for (uVar3 = uVar3 & 3; uVar3 != 0; uVar3 = uVar3 - 1) {
    *pcVar2 = *pcVar5;
    pcVar5 = pcVar5 + 1;
    pcVar2 = pcVar2 + 1;
  }
  *(undefined4 *)((int)this + 0xa0) = 0;
  *(undefined4 *)((int)this + 0xa4) = 0;
  *(undefined4 *)((int)this + 0xa8) = 0;
  *(undefined4 *)((int)this + 0xac) = 0;
  *(undefined4 *)((int)this + 0x98) = 0;
  *(undefined4 *)((int)this + 0xb0) = 0;
  DAT_006be3b8 = this;
  ExceptionList = local_c;
  return this;
}

