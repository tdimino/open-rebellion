
void __thiscall FUN_0060eed0(void *this,int param_1)

{
  char cVar1;
  HFONT pHVar2;
  int iVar3;
  uint uVar4;
  uint uVar5;
  char *pcVar6;
  LOGFONTA *pLVar7;
  char *pcVar8;
  CHAR *pCVar9;
  LOGFONTA local_3c;
  
  pLVar7 = &local_3c;
  for (iVar3 = 0xf; iVar3 != 0; iVar3 = iVar3 + -1) {
    pLVar7->lfHeight = 0;
    pLVar7 = (LOGFONTA *)&pLVar7->lfWidth;
  }
  local_3c.lfCharSet = '\0';
  local_3c.lfOutPrecision = '\0';
  local_3c.lfClipPrecision = '\0';
  uVar4 = 0xffffffff;
  local_3c.lfQuality = '\x02';
  pcVar6 = (char *)((int)this + 0x38);
  do {
    pcVar8 = pcVar6;
    if (uVar4 == 0) break;
    uVar4 = uVar4 - 1;
    pcVar8 = pcVar6 + 1;
    cVar1 = *pcVar6;
    pcVar6 = pcVar8;
  } while (cVar1 != '\0');
  uVar4 = ~uVar4;
  pcVar6 = pcVar8 + -uVar4;
  pCVar9 = local_3c.lfFaceName;
  for (uVar5 = uVar4 >> 2; uVar5 != 0; uVar5 = uVar5 - 1) {
    *(undefined4 *)pCVar9 = *(undefined4 *)pcVar6;
    pcVar6 = pcVar6 + 4;
    pCVar9 = pCVar9 + 4;
  }
  for (uVar4 = uVar4 & 3; uVar4 != 0; uVar4 = uVar4 - 1) {
    *pCVar9 = *pcVar6;
    pcVar6 = pcVar6 + 1;
    pCVar9 = pCVar9 + 1;
  }
  switch(param_1) {
  default:
    local_3c.lfHeight = 0x10;
    break;
  case 1:
  case 5:
    local_3c.lfHeight = 0x10;
    local_3c.lfWeight = 700;
    goto LAB_0060efc7;
  case 2:
    local_3c.lfHeight = 0xe;
    local_3c.lfWeight = 700;
    goto LAB_0060efc7;
  case 3:
    local_3c.lfHeight = 8;
    break;
  case 6:
    local_3c.lfHeight = 0x12;
    break;
  case 7:
    local_3c.lfHeight = 0x12;
    local_3c.lfWeight = 700;
    goto LAB_0060efc7;
  case 8:
  case 9:
    local_3c.lfHeight = 0xc;
    break;
  case 10:
    local_3c.lfHeight = 0xe;
    break;
  case 0xb:
    local_3c.lfHeight = 0x18;
    break;
  case 0xc:
    local_3c.lfHeight = 0x1e;
    break;
  case 0xd:
    local_3c.lfHeight = 0xc;
    local_3c.lfWeight = 900;
    goto LAB_0060efc7;
  }
  local_3c.lfWeight = 400;
LAB_0060efc7:
  pHVar2 = CreateFontIndirectA(&local_3c);
  *(HFONT *)((int)this + param_1 * 4) = pHVar2;
  return;
}

