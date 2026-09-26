
void * __thiscall
FUN_005eeed0(void *this,undefined4 *param_1,char *param_2,char *param_3,undefined4 *param_4,
            undefined4 *param_5)

{
  char cVar1;
  uint uVar2;
  uint uVar3;
  int iVar4;
  char *pcVar5;
  char *pcVar6;
  undefined4 *puVar7;
  
  FUN_005f5c10(this);
  *(undefined ***)this = &PTR_FUN_0066dba0;
  *(undefined4 *)((int)this + 0x18) = *param_1;
  *(undefined4 *)((int)this + 0x1c) = param_1[1];
  *(undefined4 *)((int)this + 0x20) = param_1[2];
  *(undefined4 *)((int)this + 0x24) = param_1[3];
  uVar2 = 0xffffffff;
  do {
    pcVar5 = param_3;
    if (uVar2 == 0) break;
    uVar2 = uVar2 - 1;
    pcVar5 = param_3 + 1;
    cVar1 = *param_3;
    param_3 = pcVar5;
  } while (cVar1 != '\0');
  uVar2 = ~uVar2;
  pcVar5 = pcVar5 + -uVar2;
  pcVar6 = (char *)((int)this + 0x28);
  for (uVar3 = uVar2 >> 2; uVar3 != 0; uVar3 = uVar3 - 1) {
    *(undefined4 *)pcVar6 = *(undefined4 *)pcVar5;
    pcVar5 = pcVar5 + 4;
    pcVar6 = pcVar6 + 4;
  }
  for (uVar2 = uVar2 & 3; uVar2 != 0; uVar2 = uVar2 - 1) {
    *pcVar6 = *pcVar5;
    pcVar5 = pcVar5 + 1;
    pcVar6 = pcVar6 + 1;
  }
  uVar2 = 0xffffffff;
  do {
    pcVar5 = param_2;
    if (uVar2 == 0) break;
    uVar2 = uVar2 - 1;
    pcVar5 = param_2 + 1;
    cVar1 = *param_2;
    param_2 = pcVar5;
  } while (cVar1 != '\0');
  uVar2 = ~uVar2;
  pcVar5 = pcVar5 + -uVar2;
  pcVar6 = (char *)((int)this + 0x46);
  for (uVar3 = uVar2 >> 2; uVar3 != 0; uVar3 = uVar3 - 1) {
    *(undefined4 *)pcVar6 = *(undefined4 *)pcVar5;
    pcVar5 = pcVar5 + 4;
    pcVar6 = pcVar6 + 4;
  }
  for (uVar2 = uVar2 & 3; uVar2 != 0; uVar2 = uVar2 - 1) {
    *pcVar6 = *pcVar5;
    pcVar5 = pcVar5 + 1;
    pcVar6 = pcVar6 + 1;
  }
  if (param_4[2] == 0) {
    *(undefined4 *)((int)this + 0x144) = 0;
  }
  else {
    *(undefined4 *)((int)this + 0x144) = 1;
  }
  if (*(int *)((int)this + 0x144) == 0) {
    param_4 = param_5;
  }
  puVar7 = (undefined4 *)((int)this + 0x78);
  for (iVar4 = 0x33; iVar4 != 0; iVar4 = iVar4 + -1) {
    *puVar7 = *param_4;
    param_4 = param_4 + 1;
    puVar7 = puVar7 + 1;
  }
  return this;
}

