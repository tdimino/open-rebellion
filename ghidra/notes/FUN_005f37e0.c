
void FUN_005f37e0(undefined4 param_1,undefined4 *param_2)

{
  char cVar1;
  code *pcVar2;
  uint uVar3;
  uint uVar4;
  char *pcVar5;
  char *pcVar6;
  bool bVar7;
  
  uVar3 = 0xffffffff;
  *param_2 = 0;
  pcVar5 = &DAT_006b120c;
  do {
    pcVar6 = pcVar5;
    if (uVar3 == 0) break;
    uVar3 = uVar3 - 1;
    pcVar6 = pcVar5 + 1;
    cVar1 = *pcVar5;
    pcVar5 = pcVar6;
  } while (cVar1 != '\0');
  uVar3 = ~uVar3;
  pcVar5 = pcVar6 + -uVar3;
  pcVar6 = (char *)&DAT_006bdce8;
  for (uVar4 = uVar3 >> 2; pcVar2 = DAT_006bde14, uVar4 != 0; uVar4 = uVar4 - 1) {
    *(undefined4 *)pcVar6 = *(undefined4 *)pcVar5;
    pcVar5 = pcVar5 + 4;
    pcVar6 = pcVar6 + 4;
  }
  bVar7 = DAT_006bde14 != (code *)0x0;
  for (uVar3 = uVar3 & 3; uVar3 != 0; uVar3 = uVar3 - 1) {
    *pcVar6 = *pcVar5;
    pcVar5 = pcVar5 + 1;
    pcVar6 = pcVar6 + 1;
  }
  if (bVar7) {
    (*pcVar2)(param_1,0x100,&DAT_006bdce8);
  }
  FUN_005f3650(&DAT_006bdce8,param_2);
  return;
}

