
bool __cdecl FUN_005f3850(void *param_1,int *param_2)

{
  char cVar1;
  void *this;
  ushort uVar2;
  int iVar3;
  uint uVar4;
  uint uVar5;
  char *pcVar6;
  char *pcVar7;
  
  this = param_1;
  *param_2 = 0;
  FUN_005f4c40(param_1,&param_1);
  uVar2 = FUN_00479ac0();
  iVar3 = FUN_00618b70((uint)uVar2 + ((uint)param_1 & 0xffff));
  if (iVar3 != 0) {
    FUN_006173b0(this,iVar3,(uint)param_1 & 0xffff);
    uVar4 = 0xffffffff;
    pcVar6 = &DAT_006b120c;
    do {
      pcVar7 = pcVar6;
      if (uVar4 == 0) break;
      uVar4 = uVar4 - 1;
      pcVar7 = pcVar6 + 1;
      cVar1 = *pcVar6;
      pcVar6 = pcVar7;
    } while (cVar1 != '\0');
    uVar4 = ~uVar4;
    pcVar6 = pcVar7 + -uVar4;
    pcVar7 = (char *)(iVar3 + ((uint)param_1 & 0xffff));
    for (uVar5 = uVar4 >> 2; uVar5 != 0; uVar5 = uVar5 - 1) {
      *(undefined4 *)pcVar7 = *(undefined4 *)pcVar6;
      pcVar6 = pcVar6 + 4;
      pcVar7 = pcVar7 + 4;
    }
    for (uVar4 = uVar4 & 3; uVar4 != 0; uVar4 = uVar4 - 1) {
      *pcVar7 = *pcVar6;
      pcVar6 = pcVar6 + 1;
      pcVar7 = pcVar7 + 1;
    }
    *param_2 = iVar3;
  }
  return iVar3 != 0;
}

