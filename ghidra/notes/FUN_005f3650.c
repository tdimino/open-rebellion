
undefined4 __cdecl FUN_005f3650(char *param_1,undefined4 *param_2)

{
  char cVar1;
  ushort uVar2;
  uint uVar3;
  char *pcVar4;
  uint uVar5;
  undefined4 uVar6;
  char *pcVar7;
  
  *param_2 = 0;
  uVar6 = 1;
  if (param_1 != (char *)0x0) {
    uVar3 = FUN_005f3630(param_1);
    uVar2 = FUN_00479ac0();
    pcVar4 = (char *)FUN_00618b70((uVar3 & 0xffff) + (uint)uVar2);
    *param_2 = pcVar4;
    if (pcVar4 != (char *)0x0) {
      uVar3 = 0xffffffff;
      do {
        pcVar7 = param_1;
        if (uVar3 == 0) break;
        uVar3 = uVar3 - 1;
        pcVar7 = param_1 + 1;
        cVar1 = *param_1;
        param_1 = pcVar7;
      } while (cVar1 != '\0');
      uVar3 = ~uVar3;
      pcVar7 = pcVar7 + -uVar3;
      for (uVar5 = uVar3 >> 2; uVar5 != 0; uVar5 = uVar5 - 1) {
        *(undefined4 *)pcVar4 = *(undefined4 *)pcVar7;
        pcVar7 = pcVar7 + 4;
        pcVar4 = pcVar4 + 4;
      }
      for (uVar3 = uVar3 & 3; uVar3 != 0; uVar3 = uVar3 - 1) {
        *pcVar4 = *pcVar7;
        pcVar7 = pcVar7 + 1;
        pcVar4 = pcVar4 + 1;
      }
      return 1;
    }
    uVar6 = 0;
  }
  return uVar6;
}

